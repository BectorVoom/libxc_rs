//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1385/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1385(t10908: f64, t6755: f64, t1012: f64, t10515: f64, t6753: f64, t1933: f64, t23479: f64, t82916: f64, t1025: f64, t10360: f64, t10463: f64, t10493: f64, t1929: f64, t1932: f64, t1934: f64, t1937: f64, t1941: f64, t23433: f64, t23529: f64, t23544: f64, t3057: f64, t3064: f64, t3123: f64, t3134: f64, t378: f64, t612: f64, t6765: f64, t82941: f64, t82944: f64, t82951: f64, t82953: f64, t82956: f64) -> f64 {
    let t82961 = t6755 * t10908;
    let t82964 = t1012 * t6753 * t10515;
    let t82971 = t1933 * t82916 * t23479;
    let t82979 = t23544 * t3057 / 768.0_f64 + t6765 * t10463 / 2304.0_f64 + 5.0_f64 / 2304.0_f64 * t23544 * t3064 + 0.60559134141210586284e-3_f64 * t82941 - 0.48447307312968469026e-2_f64 * t82944 + t23433 * t3123 / 512.0_f64 + t6765 * t10493 / 384.0_f64 - 0.30279567070605293142e-3_f64 * t82951 + t82953 / 384.0_f64 - t82956 * t3134 / 48.0_f64 - t23529 * t3057 / 144.0_f64 + t82961 / 768.0_f64 + 19.0_f64 / 288.0_f64 * t82964 * t1025 + t10360 * t1941 * t378 / 1536.0_f64 - 0.60559134141210586284e-3_f64 * t82971 - 0.72670960969452703541e-1_f64 / t1929 / t612 * t1932 * t1934 * t1937;
    t82979
}
