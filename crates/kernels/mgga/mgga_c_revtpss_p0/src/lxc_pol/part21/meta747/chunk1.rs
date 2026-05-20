//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2622/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2622<F: Float>(t48269: F, t47019: F, t39773: F, t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t47003: F, t48258: F, t48259: F, t48261: F, t48263: F, t48264: F, t48265: F, t48266: F, t48268: F) -> (F, F, F) {
    let t48270 = F::cast_from(0.51947577317044391277e2_f64) * t48269;
    let t48271 = F::new(960.0) * t47019;
    let t48272 = t47003 - t48258 + t48259 + t39773 + t48261 - t48263 - t39783 - t39786 - t39791 - t39795 + t48264 - t48265 - t48266 + t48268 - t48270 - t48271 + t39799;
    (t48270, t48271, t48272)
}
