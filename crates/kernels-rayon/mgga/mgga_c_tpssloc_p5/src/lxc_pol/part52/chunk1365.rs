//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1365/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1365(t115903: f64, t119891: f64, t115833: f64, t119883: f64, t119879: f64, t25994: f64, t7266: f64, t119795: f64, t119796: f64, t1458: f64, t1869: f64, t2314: f64, t27858: f64, t31829: f64, t31913: f64, t33740: f64, t33756: f64, t4028: f64, t4034: f64, t4072: f64, t650: f64, t6515: f64, t652: f64, t6862: f64, t7983: f64, t8103: f64, t8682: f64) -> (f64, f64, f64, f64) {
    let t121102 = t115903 * t119891;
    let t121105 = t115833 * t119883;
    let t121108 = t115833 * t119879;
    let t122875 = t7266 * t25994;
    let t122889 = -2.0_f64 * t1458 * t31829 * t652 - 2.0_f64 * t4072 * t652 * t8682 - t1869 * t27858 - 2.0_f64 * t2314 * t33740 - 2.0_f64 * t31913 * t4028 - 2.0_f64 * t33740 * t4034 - t33756 * t650 - t6515 * t8103 - t6862 * t7983 + t119795 - t119796 - 2.0_f64 * t122875;
    (t121102, t121105, t121108, t122889)
}
