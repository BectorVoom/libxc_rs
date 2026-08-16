//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 926/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk926<F: Float>(t547: F, t9714: F, t807: F, t9646: F, t2236: F, t66: F) -> (F, F, F, F) {
    let t9715 = t547 * t9714;
    let t9716 = t807 * t9715;
    let t9718 = t9646 * t547;
    let t9720 = F::cast_from(1.0_f64) / t66 / t2236;
    (t9715, t9716, t9718, t9720)
}
