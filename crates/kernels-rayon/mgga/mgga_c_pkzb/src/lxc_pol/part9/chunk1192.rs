//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1192/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1192(t12: f64, t2735: f64, t500: f64, t1429: f64, t17361: f64, t1837: f64, t19633: f64, t19636: f64, t19642: f64, t19645: f64, t2732: f64, t439: f64, t5094: f64, t5100: f64, t5528: f64, t652: f64, t7337: f64, t7340: f64, t8: f64, t82: f64, t972: f64, zeta_threshold: f64) -> f64 {
    let t84 = t12 <= zeta_threshold;
    let t20741 = 8.0_f64 * t2735 * t500;
    let t20743 = piecewise3(t84, 0.0_f64, 280.0_f64 / 81.0_f64 * t17361 * t972 * t5094 - 56.0_f64 / 9.0_f64 * t5528 * t8 * t19633 - 28.0_f64 / 9.0_f64 * t7337 * t19636 + 8.0_f64 / 3.0_f64 * t1837 * t1429 * t439 - 8.0_f64 * t7340 * t19642 + 8.0_f64 / 3.0_f64 * t7340 * t19645 + 4.0_f64 / 9.0_f64 * t2732 * t5100 + 4.0_f64 * t652 * t82 - t20741);
    t20743
}
