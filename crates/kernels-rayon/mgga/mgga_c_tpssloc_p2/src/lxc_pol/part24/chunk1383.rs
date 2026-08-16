//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1383/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1383(t23562: f64, t343: f64, t82921: f64, t23482: f64, t3: f64, t23563: f64, t1025: f64, t10428: f64, t10433: f64, t10444: f64, t1933: f64, t1934: f64, t1940: f64, t23437: f64, t23521: f64, t23537: f64, t23541: f64, t3008: f64, t3077: f64, t3123: f64, t354: f64, t378: f64, t6735: f64, t6747: f64, t6758: f64, t82880: f64, t82883: f64, t82885: f64, t82893: f64, t82897: f64, t82911: f64, t82914: f64, t82918: f64) -> f64 {
    let t82923 = t23562 * t82921 * t343;
    let t82926 = t23482 * t3;
    let t82927 = t82926 * t23563;
    let t82932 = -t82880 * t1025 / 48.0_f64 + t82883 / 768.0_f64 + t82885 / 432.0_f64 - t3077 * t6758 * t378 / 96.0_f64 + 0.60559134141210586284e-3_f64 * t82893 - 0.30279567070605293142e-3_f64 * t82897 - 0.30279567070605293142e-3_f64 * t1933 * t1934 * t3008 * t6735 + t23537 * t10428 / 256.0_f64 - t23541 * t10433 / 512.0_f64 - 209.0_f64 / 1296.0_f64 * t354 * t1940 * t10444 * t378 + 0.30279567070605293142e-3_f64 * t82911 * t23521 - t82914 / 2304.0_f64 - 0.60559134141210586284e-3_f64 * t82918 * t6747 - 0.30279567070605293142e-3_f64 * t82923 * t6747 + 0.48447307312968469026e-2_f64 * t82927 * t6747 - t23437 * t3123 / 96.0_f64;
    t82932
}
