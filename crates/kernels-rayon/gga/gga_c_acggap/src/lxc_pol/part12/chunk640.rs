//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 640/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk640(t2975: f64, t2984: f64, t484: f64, t709: f64, t712: f64, t715: f64, t2992: f64, t2998: f64, t1381: f64, t691: f64, t1378: f64, t75: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5030 = 80.0_f64 * t2975;
    let t5031 = 0.11696447245269292414e1_f64 * t2984;
    let t5032 = t709 * t484;
    let t5033 = 20.0_f64 * t5032;
    let t5034 = t712 * t484;
    let t5035 = 12.0_f64 * t5034;
    let t5036 = t715 * t484;
    let t5037 = 32.0_f64 * t5036;
    let t5038 = 4.0_f64 * t2992;
    let t5039 = 40.0_f64 * t2998;
    let t5040 = t1381 * t691;
    let t5041 = 0.17315859105681463759e2_f64 * t5040;
    let t5042 = t1378 * t75;
    (t5030, t5031, t5033, t5035, t5037, t5038, t5039, t5041, t5042)
}
