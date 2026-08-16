//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1380/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1380(t10955: f64, t1940: f64, t354: f64, t10459: f64, t6765: f64, t10870: f64, t10489: f64, t1046: f64, t10501: f64, t10915: f64, t10919: f64, t23529: f64, t23544: f64, t3043: f64, t3064: f64, t3098: f64, t82843: f64, t82848: f64, t82851: f64, t82859: f64, t82861: f64, t82863: f64) -> f64 {
    let t82868 = t354 * t1940 * t10955;
    let t82871 = t6765 * t10459;
    let t82875 = t6765 * t10870;
    let t82877 = t6765 * t10489;
    let t82879 = t82843 / 1152.0_f64 - 5.0_f64 / 432.0_f64 * t23529 * t3064 + t82848 * t3043 / 96.0_f64 - t82851 / 2304.0_f64 - t23544 * t3098 / 384.0_f64 - t6765 * t10915 / 384.0_f64 + 5.0_f64 / 2304.0_f64 * t6765 * t10919 + t82859 / 384.0_f64 - t82861 / 768.0_f64 - t82863 / 108.0_f64 + t23529 * t3098 / 72.0_f64 + 19.0_f64 / 432.0_f64 * t82868 * t1046 + 5.0_f64 / 3456.0_f64 * t82871 - 5.0_f64 / 1152.0_f64 * t6765 * t10501 - t82875 / 3456.0_f64 - t82877 / 576.0_f64;
    t82879
}
