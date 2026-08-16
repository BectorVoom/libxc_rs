//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 901/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk901(t1551: f64, t1554: f64, t1556: f64, t1562: f64, t1563: f64, t2259: f64, t2531: f64, t2533: f64, t2534: f64, t2538: f64, t2847: f64, t285: f64, t495: f64, t499: f64, t5078: f64, t5081: f64, t5087: f64, t7195: f64, t7197: f64, t7202: f64, t7204: f64, t7206: f64, t7218: f64, t7221: f64, t792: f64, t8296: f64, t921: f64, t983: f64) -> f64 {
    let t8299 = t7195 * t285 + 2.0_f64 * t7197 * t2534 + t2531 * t1556 / 2.0_f64 + t7202 * t2534 + t7204 * t2534 + t2533 * t7206 / 2.0_f64 - 5.0_f64 / 16.0_f64 * t921 * t5078 + t921 * t5081 / 4.0_f64 + t1551 * t2538 / 4.0_f64 + t1554 * t2538 / 4.0_f64 - 5.0_f64 / 8.0_f64 * t495 * t7218 + t495 * t7221 / 2.0_f64 + 45.0_f64 / 64.0_f64 * t5087 * t983 * t1563 - 5.0_f64 / 8.0_f64 * t1562 * t2847 * t792 - 5.0_f64 / 16.0_f64 * t1562 * t983 * t2259 + t499 * t8296 / 4.0_f64;
    t8299
}
