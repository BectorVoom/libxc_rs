//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 523/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk523<F: Float>(t2323: F, t2892: F, t97: F, t2453: F, t2454: F, t990: F, t1248: F, t1217: F, t413: F, t298: F, t302: F, t994: F, t1000: F, t1256: F, t308: F, t1001: F, t1268: F, t295: F, t305: F, t309: F, t997: F, rho1: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2894 = t97 * t2323 * t2892;
    let t2895 = 6.0 * t2894;
    let t2896 = 2.0 * t2453;
    let t2897 = 8.0 * t2454;
    let t2900 = t990 * t990;
    let t2901 = t1248 * t2900;
    let t2904 = t413 + t1217;
    let t2905 = t298 * t2904;
    let t2910 = 1.0 / t302 / t994 / rho1;
    let t2911 = tau1 * t2910;
    let t2916 = t1000 * t1000;
    let t2917 = t1256 * t2916;
    let t2920 = -t2904;
    let t2921 = t308 * t2920;
    let t2924 = 10.0 / 9.0 * t295 * t2901 + 5.0 / 3.0 * t295 * t2905 + 40.0 / 9.0 * t2911 * t309 - 50.0 / 9.0 * t997 * t1001 + 10.0 / 9.0 * t305 * t2917 + 5.0 / 3.0 * t305 * t2921 - t1268;
    (t2895, t2896, t2897, t2900, t2901, t2904, t2905, t2911, t2916, t2917, t2920, t2921, t2924)
}
