//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 613/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk613(t3566: f64, t487: f64, t1209: f64, t1269: f64, t3356: f64, t3140: f64, t460: f64, t1242: f64, t472: f64, t474: f64, t3147: f64, t479: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3567 = t3566 * t487;
    let t3572 = t1209 * t1269;
    let t3579 = 0.19755555555555555556e-1_f64 * t3356;
    let t3594 = t460 * t3140;
    let t3596 = 1.0_f64 / t1242 / t472;
    let t3597 = t3596 * t474;
    let t3598 = t479 * t3147;
    (t3567, t3572, t3579, t3594, t3596, t3597, t3598)
}
