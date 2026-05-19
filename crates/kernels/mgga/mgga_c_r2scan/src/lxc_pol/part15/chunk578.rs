//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 578/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk578<F: Float>(t1604: F, t2842: F, t2169: F, t2220: F, t2231: F, t2236: F, t2683: F, t2685: F, t2689: F, t2693: F, t2696: F, t2699: F, t2721: F, t2728: F, t2732: F, t279: F, t2834: F, t2839: F, t527: F, t549: F, t940: F, t944: F) -> F {
    let t2843 = t1604 * t2842;
    let t2845 = -F::cast_from(0.97574405393827830187e-2_f64) * t2683 + F::cast_from(0.27439371595564631661e-2_f64) * t2685 - F::cast_from(0.58218257753910989057e-2_f64) * t2689 - F::cast_from(0.17465477326173296717e-1_f64) * t2693 + F::cast_from(0.34672886960217074253e0_f64) * t2220 - F::cast_from(0.11557628986739024751e0_f64) * t2696 - F::cast_from(0.54878743191129263322e-1_f64) * t527 * t2699 - F::cast_from(0.43341108700271342816e-1_f64) * t2236 * t940 + t2231 - F::cast_from(0.43341108700271342816e-1_f64) * t549 * t2721 - F::cast_from(0.13002332610081402845e0_f64) * t2169 * t944 + F::cast_from(0.11557628986739024751e0_f64) * t2728 + F::cast_from(0.34672886960217074253e0_f64) * t2732 + F::cast_from(0.43341108700271342816e-1_f64) * t2834 * t279 + F::cast_from(0.58218257753910989057e-2_f64) * t2839 + F::cast_from(0.54878743191129263322e-2_f64) * t2843;
    t2845
}
