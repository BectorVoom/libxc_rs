//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 776/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk776<F: Float>(t12915: F, t828: F, t1242: F, t11239: F, t1243: F, t3596: F, t1275: F, t4171: F, t602: F, t1466: F, t2246: F) -> (F, F, F, F, F, F, F, F) {
    let t12916 = t828 * t12915;
    let t13037 = t1242 * t1242;
    let t13038 = F::new(1.0) / t13037;
    let t13126 = t11239 * t1243;
    let t13141 = t11239 * t3596;
    let t13180 = t1275 * t1275;
    let t13181 = F::new(1.0) / t13180;
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    (t12916, t13038, t13126, t13141, t13180, t13181, t13269, t13272)
}
