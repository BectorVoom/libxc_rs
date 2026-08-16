//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2327/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2327(t16596: f64, t89992: f64, t23788: f64, t98007: f64, t17109: f64, t28: f64, t25365: f64, t98058: f64, t25927: f64, t98003: f64, t1081: f64, t1877: f64, t22959: f64, t23290: f64, t25013: f64, t2522: f64, t25354: f64, t25358: f64, t25930: f64, t25934: f64, t28448: f64, t28774: f64, t28792: f64, t28795: f64, t6666: f64, t6670: f64, t7649: f64, t7656: f64, t86836: f64, t99055: f64) -> f64 {
    let t100766 = t89992 * t16596;
    let t100769 = t23788 * t98007;
    let t100772 = t28 * t17109;
    let t100780 = t89992 * t25365;
    let t100788 = t23788 * t98058;
    let t100791 = t25927 * t98003;
    let t100803 = -t1877 * t86836 * t7656 - 3.0_f64 * t22959 * t100766 - 3.0_f64 * t22959 * t100769 - t1877 * t6670 * t100772 / 2.0_f64 - t1877 * t25358 * t25934 - t1877 * t25358 * t25930 - 3.0_f64 * t22959 * t100780 + 3.0_f64 * t2522 * t25354 * t7649 - t1877 * t23290 * t28792 - t99055 - 6.0_f64 * t25013 * t100788 + 3.0_f64 * t22959 * t100791 + t1877 * t28448 * t1081 / 2.0_f64 + 3.0_f64 * t2522 * t6666 * t28774 - t1877 * t23290 * t28795 / 2.0_f64;
    t100803
}
