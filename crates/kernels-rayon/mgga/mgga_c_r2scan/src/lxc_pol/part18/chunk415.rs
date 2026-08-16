//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 415/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk415(t1923: f64, t686: f64, t1803: f64, t1939: f64, t1942: f64, t1945: f64, t1946: f64, t1949: f64, t1957: f64, t1966: f64, t201: f64, t207: f64, t208: f64, t390: f64, t664: f64, t674: f64, t682: f64, t687: f64, t689: f64, t690: f64, t705: f64) -> f64 {
    let t1973 = t686 * t1923;
    let t1976 = -0.11015083824637807018e1_f64 * t390 * t1939 - 0.11696447245269292414e1_f64 * t705 * t1942 - 0.10389515463408878255e3_f64 * t1945 * t1946 - 0.23392894490538584828e1_f64 * t705 * t1949 + 6.0_f64 * t687 * t208 * t1923 - 0.19298375398431042081e3_f64 * t1957 * t690 * t1923 - 4.0_f64 * t674 * t682 * t664 - 2.0_f64 * t674 * t208 * t1966 + 0.20548e0_f64 * t201 * t1966 * t207 + 0.66090502947826842111e1_f64 * t1973 * t689 - t1803;
    t1976
}
