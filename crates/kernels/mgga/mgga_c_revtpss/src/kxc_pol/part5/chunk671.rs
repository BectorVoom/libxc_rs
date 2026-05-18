//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 671/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk671<F: Float>(t4306: F, t190: F, t4186: F, t706: F, t1531: F, t705: F) -> (F, F, F, F) {
    let t4307 = F::new(4.0) * t4306;
    let t4308 = t190 * t4186;
    let t4310 = F::new(4.0) * t706 * t4308;
    let t4311 = t705 * t1531;
    (t4307, t4308, t4310, t4311)
}
