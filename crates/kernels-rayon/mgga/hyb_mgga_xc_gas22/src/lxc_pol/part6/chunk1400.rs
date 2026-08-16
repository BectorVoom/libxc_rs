//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1400/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1400(t1014: f64, t10865: f64, t10868: f64, t11173: f64, t11174: f64, t2579: f64, t2609: f64, t30127: f64, t30182: f64, t30184: f64, t30198: f64, t30200: f64, t30203: f64, t3591: f64, t7165: f64, t9282: f64, t9289: f64) -> f64 {
    let t30348 = t30127 + t30182 + t30184 + 0.10389515463408878255e3_f64 * t1014 * t11173 * t7165 + 0.14035736694323150897e2_f64 * t1014 * t11173 * t2579 - 0.69263436422725855034e2_f64 * t3591 * t9289 - 0.34631718211362927518e2_f64 * t2609 * t10865 + 0.23392894490538584828e1_f64 * t3591 * t9282 - 0.69263436422725855036e2_f64 * t2609 * t10868 + 0.2077903092681775651e3_f64 * t2609 * t11174 + t30198 + t30200 + t30203;
    t30348
}
