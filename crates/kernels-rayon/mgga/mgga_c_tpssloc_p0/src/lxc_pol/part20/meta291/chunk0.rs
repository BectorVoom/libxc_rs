//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1500/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1500(t10913: f64, t4583: f64, t4582: f64, t4588: f64, t698: f64, t999: f64, t973: f64, t2960: f64, t3139: f64, t1000: f64, t1020: f64, t1025: f64, t10263: f64, t1041: f64, t1046: f64, t10517: f64, t10860: f64, t10863: f64, t10866: f64, t10871: f64, t10873: f64, t10876: f64, t10879: f64, t10883: f64, t10886: f64, t10891: f64, t10896: f64, t10898: f64, t10904: f64, t10909: f64, t3043: f64, t3057: f64, t3109: f64, t3117: f64, t3123: f64, t3134: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10914 = t4583 * t10913;
    let t10915 = t4582 * t10914;
    let t10918 = t4588 * t10913;
    let t10919 = t4582 * t10918;
    let t10922 = t698 * t999;
    let t10923 = t973 * t10922;
    let t10927 = t2960 * t3139;
    let t10929 = 19.0_f64 / 576.0_f64 * t10517 * t1025 + t1020 * t10860 / 3072.0_f64 - t10863 * t1046 / 144.0_f64 + t10866 / 1152.0_f64 - t10871 / 6912.0_f64 - t10873 / 216.0_f64 - t10876 * t10879 / 512.0_f64 + t10883 * t10886 / 3072.0_f64 + t10891 * t3043 / 192.0_f64 - t10896 / 1536.0_f64 - t10898 * t1025 / 96.0_f64 - t3109 * t3123 / 192.0_f64 - t10904 * t3134 / 96.0_f64 + t10909 / 1536.0_f64 + t3117 * t3057 / 1536.0_f64 - t1041 * t10915 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t1041 * t10919 - t10923 / 432.0_f64 + 11.0_f64 / 108.0_f64 * t10263 * t1000 - t10927 / 54.0_f64;
    (t10914, t10915, t10918, t10919, t10922, t10923, t10927, t10929)
}
