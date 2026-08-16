//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 303/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk303(t2415: f64, t291: f64, t2211: f64, t570: f64, t2217: f64, t2220: f64, t2223: f64, t2226: f64, t2348: f64, t2351: f64, t2354: f64, t2357: f64, t2359: f64, t2361: f64, t2363: f64, t2365: f64) -> (f64, f64, f64) {
    let t2416 = t2415 * t291;
    let t2435 = t2211 * t570;
    let t2447 = -0.19957069503106347607e-1_f64 * t2348 + 0.2993560425465952141e-1_f64 * t2351 + t2217 + 0.68186654135613354324e-2_f64 * t2354 - 0.90915538847484472432e-2_f64 * t2357 - t2220 - 0.66380770525302906695e-3_f64 * t2359 + 0.79656924630363488034e-3_f64 * t2361 + t2223 + 0.1814407727691612783e-3_f64 * t2363 - 0.21168090156402149135e-3_f64 * t2365 - t2226;
    (t2416, t2435, t2447)
}
