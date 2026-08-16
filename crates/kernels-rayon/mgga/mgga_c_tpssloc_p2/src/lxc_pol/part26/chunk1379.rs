//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1379/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1379(t7288: f64, t85660: f64, t225: f64, t24758: f64, t24637: f64, t7294: f64, t2121: f64, t3427: f64, t7295: f64, t11598: f64, t1186: f64, t11868: f64, t11928: f64, t11934: f64, t11935: f64, t1238: f64, t1251: f64, t1252: f64, t2144: f64, t2155: f64, t24615: f64, t24867: f64, t24893: f64, t3481: f64, t3598: f64, t3600: f64, t44412: f64, t462: f64, t497: f64, t498: f64, t7283: f64, t7300: f64, t7348: f64, t7351: f64, t7392: f64) -> f64 {
    let t86473 = t85660 * t7288;
    let t86475 = t24758 * t225;
    let t86494 = t7294 * t24637;
    let t86501 = t2121 * t3427 * t7295;
    let t86506 = 0.49348022005446793095e-1_f64 * t7283 * t7300 * t24615 * t11934 + 0.18277045187202515961e-2_f64 * t86473 - 3.0_f64 * t86475 * t1252 + t11598 * t2144 * t498 + 3.0_f64 * t3481 * t7348 * t498 + 6.0_f64 * t1238 * t3598 * t24867 * t1251 + 0.82246703342411321825e-2_f64 * t2121 * t462 * t11868 * t225 * t497 + 6.0_f64 * t24893 * t3600 + 0.49348022005446793095e-1_f64 * t7283 * t1186 * t86494 + 6.0_f64 * t7351 * t11935 - 0.54831135561607547884e-2_f64 * t86501 - 3.0_f64 * t11928 * t7392 - t44412 * t2155;
    t86506
}
