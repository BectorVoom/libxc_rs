//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1633/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1633(t1263: f64, t3588: f64, t372: f64, t1222: f64, t12809: f64, t12855: f64, t13069: f64, t17344: f64, t17351: f64, t17354: f64, t17693: f64, t17694: f64, t247: f64, t3591: f64, t3604: f64, t3611: f64, t3719: f64, t3720: f64, t3723: f64, t43839: f64, t44759: f64, t44769: f64, t44773: f64, t44776: f64, t44778: f64, t44786: f64, t44789: f64, t44792: f64, t44797: f64, t44800: f64, t5312: f64) -> (f64, f64) {
    let t44808 = t372 * t1263 * t3588;
    let t44812 = -0.25724410870841842184e-2_f64 * t12855 * t3720 * t44759 * t3604 + 0.12862205435420921092e-2_f64 * t12809 * t3720 * t44759 * t3611 - 0.25724410870841842184e-2_f64 * t44769 * t3723 - 0.17149607247227894789e-2_f64 * t44773 + 0.34299214494455789578e-2_f64 * t44776 - 0.77173232612525526552e-2_f64 * t17344 * t247 * t3719 * t44778 + 0.12862205435420921092e-2_f64 * t13069 * t3591 + t44786 / 54.0_f64 + 0.19055119163586549765e-2_f64 * t44789 + 0.11433071498151929859e-2_f64 * t44792 - t44797 + 0.28582678745379824648e-2_f64 * t17693 * t17694 * t44800 + t1222 * t5312 * t43839 / 54.0_f64 + 0.17149607247227894789e-2_f64 * t17351 * t44808 * t17354;
    (t44808, t44812)
}
