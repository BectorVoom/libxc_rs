//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2358/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2358(t1983: f64, t22591: f64, t24990: f64, t24987: f64, t6880: f64, t22573: f64, t7684: f64, t22575: f64, t22585: f64, t7685: f64, t12725: f64, t12813: f64, t1976: f64, t22483: f64, t2312: f64, t2314: f64, t2323: f64, t24983: f64, t24999: f64, t25958: f64, t3652: f64, t4026: f64, t4028: f64, t650: f64, t652: f64, t6539: f64, t671: f64, t6862: f64, t7451: f64, t7670: f64, t91623: f64, t91625: f64, t91627: f64, t91630: f64, t91637: f64) -> f64 {
    let t91640 = 6.0_f64 * t1983 * t22591 * t24990;
    let t91642 = 6.0_f64 * t24987 * t6880;
    let t91655 = t7684 * t22573;
    let t91657 = 6.0_f64 * t91655 * t22575;
    let t91662 = 3.0_f64 * t7685 * t22585;
    let t91663 = -2.0_f64 * t12813 * t1976 * t652 - 4.0_f64 * t25958 * t652 * t671 - 4.0_f64 * t12725 * t6539 - 2.0_f64 * t22483 * t4028 - t2312 * t7670 - 4.0_f64 * t2314 * t24983 - 4.0_f64 * t2323 * t24999 - 2.0_f64 * t25958 * t650 - t3652 * t7451 - 2.0_f64 * t4026 * t6862 + t91623 - t91625 - t91627 - t91630 + t91637 + t91640 + t91642 - t91657 + t91662;
    t91663
}
