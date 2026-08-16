//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2358/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2358(t104977: f64, t1442: f64, t1459: f64, t19451: f64, t20109: f64, t24932: f64, t27858: f64, t27863: f64, t27888: f64, t29848: f64, t4037: f64, t4072: f64, t4073: f64, t5460: f64, t650: f64, t652: f64, t7266: f64, t7271: f64, t8103: f64, t97792: f64, t97794: f64, t97796: f64, t97798: f64, t97800: f64, t97802: f64, t97805: f64, t97808: f64, t97811: f64) -> f64 {
    let t105045 = -4.0_f64 * t4072 * t652 * t8103 - 4.0_f64 * t104977 * t1459 - 2.0_f64 * t1442 * t27858 - 2.0_f64 * t19451 * t7271 - 4.0_f64 * t20109 * t7266 - 4.0_f64 * t24932 * t5460 - 4.0_f64 * t27863 * t4037 - 4.0_f64 * t27863 * t4073 - 4.0_f64 * t27888 * t5460 - t29848 * t650 - t97792 + t97794 - t97796 - t97798 - t97800 - t97802 + t97805 - t97808 - t97811;
    t105045
}
