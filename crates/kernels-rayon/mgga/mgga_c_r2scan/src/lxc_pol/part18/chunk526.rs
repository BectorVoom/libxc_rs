//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 526/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk526(t1604: f64, t2842: f64, t2169: f64, t2220: f64, t2231: f64, t2236: f64, t2683: f64, t2685: f64, t2689: f64, t2693: f64, t2696: f64, t2699: f64, t2721: f64, t2728: f64, t2732: f64, t279: f64, t2834: f64, t2839: f64, t527: f64, t549: f64, t940: f64, t944: f64) -> (f64, f64) {
    let t2843 = t1604 * t2842;
    let t2845 = -0.97574405393827830187e-2_f64 * t2683 + 0.27439371595564631661e-2_f64 * t2685 - 0.58218257753910989057e-2_f64 * t2689 - 0.17465477326173296717e-1_f64 * t2693 + 0.34672886960217074253e0_f64 * t2220 - 0.11557628986739024751e0_f64 * t2696 - 0.54878743191129263322e-1_f64 * t527 * t2699 - 0.43341108700271342816e-1_f64 * t2236 * t940 + t2231 - 0.43341108700271342816e-1_f64 * t549 * t2721 - 0.13002332610081402845e0_f64 * t2169 * t944 + 0.11557628986739024751e0_f64 * t2728 + 0.34672886960217074253e0_f64 * t2732 + 0.43341108700271342816e-1_f64 * t2834 * t279 + 0.58218257753910989057e-2_f64 * t2839 + 0.54878743191129263322e-2_f64 * t2843;
    (t2843, t2845)
}
