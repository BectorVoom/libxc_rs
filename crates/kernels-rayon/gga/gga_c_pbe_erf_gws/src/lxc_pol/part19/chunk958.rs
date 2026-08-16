//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 958/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk958(t3426: f64, t395: f64, t3430: f64, t10762: f64, t571: f64, t11: f64, t10785: f64, t10789: f64, t10793: f64, t10797: f64, t10801: f64, t10804: f64, t10807: f64, t10810: f64, t10813: f64, t10816: f64, t10819: f64, t10823: f64, t25: f64, t2718: f64, t7407: f64, t7409: f64) -> (f64, f64, f64, f64) {
    let t10825 = t395 * t3426;
    let t10827 = t395 * t3430;
    let t10829 = t571 * t10762;
    let t10830 = t11 * t10829;
    let t10832 = -0.29629629629629629629e-2_f64 * t25 * t10785 - 0.88888888888888888888e-2_f64 * t2718 * t10789 - 0.39999999999999999999e-1_f64 * t25 * t10793 + 0.53333333333333333332e-1_f64 * t2718 * t10797 - 0.39990740740740740742e-1_f64 * t10801 + 0.14396666666666666667e0_f64 * t10804 - 0.9597777777777777778e-1_f64 * t10807 - 0.21595e0_f64 * t10810 + 0.28793333333333333334e0_f64 * t10813 - 0.23994444444444444445e-1_f64 * t10816 + 0.71983333333333333334e-1_f64 * t10819 - 0.14814814814814814815e-1_f64 * t7407 + 0.17777777777777777778e-1_f64 * t7409 + 0.79981481481481481483e-2_f64 * t10823 - 0.23994444444444444445e-1_f64 * t10825 + 0.11997222222222222222e-1_f64 * t10827 - 0.35991666666666666667e-1_f64 * t10830;
    (t10825, t10827, t10830, t10832)
}
