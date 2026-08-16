//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 868/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk868(t14125: f64, t21708: f64, t8503: f64, t21709: f64, t8507: f64, t15384: f64, t34847: f64, t1971: f64, t2123: f64, t515: f64, t615: f64, t7230: f64) -> (f64, f64, f64, f64) {
    let t75533 = t21708 * t14125 * t8503;
    let t75536 = t21708 * t21709 * t8507;
    let t75539 = 0.1064114997332445985e-4_f64 * t34847 * t15384;
    let t75545 = 0.1064114997332445985e-4_f64 * t7230 * t1971 * t515 * t2123 * t615;
    (t75533, t75536, t75539, t75545)
}
