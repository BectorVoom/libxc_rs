//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 846/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk846(t2575: f64, t2719: f64, t3491: f64, t639: f64, t192: f64, t3396: f64, t135: f64, t144: f64, t1535: f64, t2536: f64, t2537: f64, t2706: f64, t2714: f64, t2718: f64, t4996: f64, t5005: f64, t5011: f64, t5019: f64, t5022: f64, t5154: f64, t560: f64, t568: f64, t7030: f64, t7037: f64, t7042: f64, t8795: f64, t8817: f64, t8842: f64, t8843: f64, t8844: f64, t9099: f64) -> (f64, f64, f64) {
    let t9103 = t2719 * t2575;
    let t9112 = t3491 * t639;
    let t9116 = t192 * t3396;
    let t9120 = t135 * t144 * t639 * t9099 + 3.0_f64 * t135 * t560 * t8817 + 6.0_f64 * t1535 * t2575 * t2714 + 3.0_f64 * t1535 * t568 * t9112 - 2.0_f64 * t2536 * t2537 * t2706 + 6.0_f64 * t2718 * t568 * t9116 + 12.0_f64 * t2718 * t9103 + t4996 + t5005 - t5011 + t5019 - t5022 - t5154 + t7030 - t7037 - t7042 - t8795 - t8842 - t8843 + t8844;
    (t9112, t9116, t9120)
}
