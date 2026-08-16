//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 940/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk940(t10578: f64, t11: f64, t3466: f64, t395: f64, t3470: f64, t3474: f64, t10513: f64, t625: f64, t10561: f64, t10564: f64, t10567: f64, t10570: f64, t10573: f64, t10576: f64, t5047: f64, t5082: f64, t7279: f64, t7280: f64, t7288: f64, t7290: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10579 = t11 * t10578;
    let t10581 = t395 * t3466;
    let t10583 = t395 * t3470;
    let t10585 = t395 * t3474;
    let t10587 = t625 * t10513;
    let t10588 = t11 * t10587;
    let t10591 = -0.39990740740740740742e-1_f64 * t10561 + 0.14396666666666666667e0_f64 * t10564 + 0.9597777777777777778e-1_f64 * t10567 - 0.21595e0_f64 * t10570 - 0.28793333333333333334e0_f64 * t10573 - 0.23994444444444444445e-1_f64 * t10576 + 0.71983333333333333334e-1_f64 * t10579 - t5047 - t5082 + 0.79981481481481481483e-2_f64 * t10581 - 0.23994444444444444445e-1_f64 * t10583 + 0.11997222222222222222e-1_f64 * t10585 - 0.35991666666666666667e-1_f64 * t10588 + t7279 - 0.47988888888888888888e-1_f64 * t7280 - t7288 + t7290;
    (t10579, t10581, t10583, t10585, t10588, t10591)
}
