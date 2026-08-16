//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 429/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk429(t362: f64, t368: f64, t354: f64, t1927: f64, t1935: f64, t1937: f64, t378: f64, sigma0: f64) -> (f64, f64, f64, f64) {
    let t1940 = t362 * sigma0;
    let t1941 = t1940 * t368;
    let t1942 = t354 * t1941;
    let t1945 = t1927 / 96.0_f64 + 0.10093189023535097714e-3_f64 * t1935 * t1937 + t1942 * t378 / 1536.0_f64;
    (t1940, t1941, t1942, t1945)
}
