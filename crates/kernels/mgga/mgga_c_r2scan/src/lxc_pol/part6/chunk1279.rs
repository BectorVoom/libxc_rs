//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1279/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1279<F: Float>(t607: F, t7007: F, t468: F, t7803: F, t1419: F, t2484: F, t410: F, t7008: F, t2452: F, t2850: F, t6897: F, t1048: F, t2330: F, t2463: F, t5018: F, t19694: F, t19698: F, t19702: F, t19748: F, t23972: F, t23973: F, t881: F) -> (F, F, F, F, F, F, F, F) {
    let t23976 = t7007 * t607;
    let t23979 = t7803 * t468;
    let t23980 = 0.17544670867903938621e1 * t23979;
    let t23981 = t1419 * t2484;
    let t23982 = 36.0 * t23981;
    let t23983 = t410 * t7008;
    let t23984 = 12.0 * t23983;
    let t23985 = t1419 * t2452;
    let t23986 = 36.0 * t23985;
    let t23987 = t2850 * t6897;
    let t23990 = 6.0 * t1048 * t23987 * t2330;
    let t23991 = t2463 * t5018;
    let t23992 = 0.56968947174242584612e-3 * t23991;
    let t23993 = t23972 + t19748 - 0.2363e1 * t881 * t23973 - 0.7089e1 * t881 * t23976 + t23980 - t23982 + t23984 + t19694 - t19698 - t23986 - t19702 - t23990 + t23992;
    (t23976, t23980, t23982, t23984, t23986, t23990, t23992, t23993)
}
