//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1633/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1633<F: Float>(t1263: F, t3588: F, t372: F, t1222: F, t12809: F, t12855: F, t13069: F, t17344: F, t17351: F, t17354: F, t17693: F, t17694: F, t247: F, t3591: F, t3604: F, t3611: F, t3719: F, t3720: F, t3723: F, t43839: F, t44759: F, t44769: F, t44773: F, t44776: F, t44778: F, t44786: F, t44789: F, t44792: F, t44797: F, t44800: F, t5312: F) -> (F, F) {
    let t44808 = t372 * t1263 * t3588;
    let t44812 = -F::cast_from(0.25724410870841842184e-2_f64) * t12855 * t3720 * t44759 * t3604 + F::cast_from(0.12862205435420921092e-2_f64) * t12809 * t3720 * t44759 * t3611 - F::cast_from(0.25724410870841842184e-2_f64) * t44769 * t3723 - F::cast_from(0.17149607247227894789e-2_f64) * t44773 + F::cast_from(0.34299214494455789578e-2_f64) * t44776 - F::cast_from(0.77173232612525526552e-2_f64) * t17344 * t247 * t3719 * t44778 + F::cast_from(0.12862205435420921092e-2_f64) * t13069 * t3591 + t44786 / F::new(54.0) + F::cast_from(0.19055119163586549765e-2_f64) * t44789 + F::cast_from(0.11433071498151929859e-2_f64) * t44792 - t44797 + F::cast_from(0.28582678745379824648e-2_f64) * t17693 * t17694 * t44800 + t1222 * t5312 * t43839 / F::new(54.0) + F::cast_from(0.17149607247227894789e-2_f64) * t17351 * t44808 * t17354;
    (t44808, t44812)
}
