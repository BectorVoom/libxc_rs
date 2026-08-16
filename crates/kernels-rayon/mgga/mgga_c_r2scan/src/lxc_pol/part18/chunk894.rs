//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 894/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk894(t1556: f64, t1562: f64, t2531: f64, t2533: f64, t2534: f64, t2538: f64, t2847: f64, t285: f64, t3053: f64, t3056: f64, t3060: f64, t3229: f64, t495: f64, t499: f64, t5087: f64, t7218: f64, t7221: f64, t792: f64, t8692: f64, t8694: f64, t8698: f64, t8701: f64, t8707: f64, t8714: f64, t8723: f64, t921: f64, t9560: f64, t983: f64) -> f64 {
    let t9563 = t8692 * t285 + t8694 * t2534 + t3053 * t1556 / 4.0_f64 + 2.0_f64 * t921 * t8698 + t8701 * t2534 + t3056 * t1556 / 4.0_f64 + t2531 * t2538 / 2.0_f64 + t2533 * t8707 / 2.0_f64 - 5.0_f64 / 8.0_f64 * t921 * t7218 + t921 * t7221 / 2.0_f64 - 5.0_f64 / 16.0_f64 * t495 * t8714 + 45.0_f64 / 64.0_f64 * t5087 * t3060 * t792 - 5.0_f64 / 8.0_f64 * t1562 * t983 * t2847 + t495 * t8723 / 4.0_f64 - 5.0_f64 / 16.0_f64 * t1562 * t3229 * t792 + t499 * t9560 / 4.0_f64;
    t9563
}
