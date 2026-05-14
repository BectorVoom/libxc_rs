//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1195/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1195<F: Float>(t1894: F, t21066: F, t5672: F, t190: F, t21085: F, t632: F, t1907: F, t21062: F, t653: F, t1691: F, t1917: F, t1923: F, t1956: F, t2006: F, t2008: F, t2009: F, t2029: F, t21115: F, t21416: F, t21621: F, t21963: F, t21969: F, t21972: F, t21976: F, t21985: F, t5530: F, t5632: F, t5695: F, t5829: F, t718: F, t721: F) -> (F, F, F, F) {
    let t21988 = 0.62071215503128080361e5 * t5672 * t1894 * t21066;
    let t22000 = 2.0 * t632 * t190 * t21085;
    let t22003 = 0.2894756309764656312e3 * t1907 * t653 * t21062;
    let t22004 = -t21963 + 0.21053605041484726346e2 * t718 * t1917 * t1691 - t21969 - t21972 - t21976 - 0.4155806185363551302e4 * t5530 * t721 * t21115 + 0.24828486201251232145e6 * t5695 * t2009 * t21416 - t21985 - t21988 + 0.12414243100625616072e5 * t2006 * t2029 * t2008 * t1923 - 0.164384e1 * t5829 * t21621 - 0.2379258106121766316e3 * t1956 * t1923 * t5632 + t22000 + t22003;
    (t21988, t22000, t22003, t22004)
}
