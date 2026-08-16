//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 937/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk937(t2678: f64, t34: f64, t1714: f64, t10353: f64, t657: f64, t10357: f64, t1697: f64, t3354: f64, t422: f64, t1642: f64, t10514: f64, t10517: f64, t10519: f64, t10521: f64, t10525: f64, t10536: f64, t25: f64, t2718: f64, t5052: f64, t5083: f64, t7237: f64, t7239: f64, t7269: f64, t7272: f64) -> (f64, f64, f64, f64) {
    let t10539 = t2678 * t34;
    let t10540 = t1714 * t10539;
    let t10543 = t657 * t10353;
    let t10546 = t657 * t10357;
    let t10549 = t1697 * t3354;
    let t10550 = t10549 * t422;
    let t10551 = t657 * t10550;
    let t10554 = t1642 * t3354;
    let t10555 = t10554 * t422;
    let t10556 = t1714 * t10555;
    let t10559 = -0.66666666666666666667e-2_f64 * t25 * t10514 + 0.14814814814814814815e-2_f64 * t10517 - 0.88888888888888888887e-2_f64 * t10519 + 0.44444444444444444445e-2_f64 * t10521 + 0.13333333333333333333e-1_f64 * t25 * t10525 - 0.17777777777777777778e-1_f64 * t7237 - 0.14814814814814814815e-1_f64 * t7239 - 0.74074074074074074073e-2_f64 * t5052 - 0.15996296296296296296e-1_f64 * t5083 - 0.31992592592592592592e-1_f64 * t7269 - 0.47988888888888888888e-1_f64 * t7272 - 0.29629629629629629629e-2_f64 * t25 * t10536 + 0.88888888888888888888e-2_f64 * t2718 * t10540 - 0.39999999999999999999e-1_f64 * t25 * t10543 - 0.53333333333333333332e-1_f64 * t2718 * t10546 + 0.13333333333333333333e-1_f64 * t25 * t10551 - 0.22222222222222222222e-2_f64 * t25 * t10556;
    (t10539, t10550, t10555, t10559)
}
