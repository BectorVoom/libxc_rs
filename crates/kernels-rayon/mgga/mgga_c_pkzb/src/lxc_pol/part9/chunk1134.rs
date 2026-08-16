//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1134/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1134(t12: f64, t439: f64, t82: f64, t1429: f64, t1646: f64, t2543: f64, t500: f64, t16232: f64, t1642: f64, t19633: f64, t19636: f64, t2540: f64, t5093: f64, t5094: f64, t5100: f64, t6767: f64, t6770: f64, t8: f64, t87: f64, t972: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t19642 = t82 * t439;
    let t19645 = t1429 * t1646;
    let t19653 = 32.0_f64 * t2543 * t500;
    let t19655 = piecewise3(t84, 0.0_f64, 40.0_f64 / 81.0_f64 * t16232 * t972 * t5094 - 16.0_f64 / 9.0_f64 * t5093 * t8 * t19633 - 8.0_f64 / 9.0_f64 * t6767 * t19636 + 8.0_f64 / 3.0_f64 * t1642 * t1429 * t439 - 8.0_f64 * t6770 * t19642 + 8.0_f64 / 3.0_f64 * t6770 * t19645 + 4.0_f64 / 9.0_f64 * t2540 * t5100 - 16.0_f64 * t87 * t82 + t19653);
    (t19642, t19645, t19655)
}
