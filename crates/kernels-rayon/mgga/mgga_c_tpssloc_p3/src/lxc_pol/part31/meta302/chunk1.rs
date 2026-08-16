//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1190/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1190(t10478: f64, t3128: f64, t10472: f64, t1015: f64, t1030: f64, t3036: f64, t3033: f64, t698: f64, t999: f64, t973: f64, t363: f64, t3068: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10875 = t3128 * t10478;
    let t10876 = t10472 * t10875;
    let t10882 = t1015 * t10478;
    let t10883 = t10472 * t10882;
    let t10889 = t1030 * t3036;
    let t10890 = t1015 * t10889;
    let t10891 = t3033 * t10890;
    let t10903 = t3128 * t10889;
    let t10904 = t3033 * t10903;
    let t10922 = t698 * t999;
    let t10923 = t973 * t10922;
    let t10935 = t363 * t1030;
    let t10936 = t10935 * t3068;
    (t10876, t10883, t10891, t10904, t10923, t10936)
}
