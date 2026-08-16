//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 976/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk976(t5: f64, t13450: f64, t117: f64, t4637: f64, t623: f64, t5314: f64, t645: f64, t1163: f64, t4674: f64, t1600: f64, t3537: f64, t1338: f64, t4341: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t13451 = piecewise3(t8, 0.0_f64, t13450);
    let t13452 = t13451 * t117;
    let t13458 = t623 * t4637;
    let t13463 = t5314 * t645;
    let t13470 = t1163 * t4674;
    let t13473 = t1600 * t3537;
    let t13478 = t4341 * t1338;
    (t13451, t13452, t13458, t13463, t13470, t13473, t13478)
}
