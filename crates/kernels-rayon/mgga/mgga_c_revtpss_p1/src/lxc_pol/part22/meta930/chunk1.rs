//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3158/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3158(t12916: f64, t17747: f64, t17749: f64, t1222: f64, t16725: f64, t17471: f64, t16729: f64, t13017: f64, t5373: f64, t44546: f64, t5331: f64, t5334: f64) -> (f64, f64, f64, f64, f64) {
    let t57191 = t17747 * t12916 * t17749;
    let t57209 = t1222 * t17471 * t16725;
    let t57212 = t1222 * t17471 * t16729;
    let t57214 = t5373 * t13017;
    let t57222 = t5331 * t44546 * t5334;
    (t57191, t57209, t57212, t57214, t57222)
}
