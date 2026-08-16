//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2358/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2358(t13042: f64, t13053: f64, t13065: f64, t1492: f64, t1519: f64, t1528: f64, t16804: f64, t17022: f64, t17056: f64, t17090: f64, t20936: f64, t21034: f64, t21050: f64, t218: f64, t25168: f64, t259: f64, t2597: f64, t2713: f64, t4265: f64, t4301: f64, t46488: f64, t5558: f64, t5637: f64, t5658: f64, t58143: f64, t68211: f64, t852: f64) -> f64 {
    let t68365 = 3.0_f64 * t1492 * t17022 * t259 + 3.0_f64 * t1519 * t16804 * t259 - 18.0_f64 * t17056 * t25168 * t46488 + t20936 * t259 * t852 + t218 * t259 * t68211 + 3.0_f64 * t259 * t4265 * t5558 - 3.0_f64 * t13042 * t5658 + 6.0_f64 * t13053 * t5637 - 3.0_f64 * t13053 * t5658 - 3.0_f64 * t13065 * t5658 - 3.0_f64 * t1528 * t58143 - 3.0_f64 * t17090 * t4301 - t21034 * t2597 - 6.0_f64 * t21050 * t2713;
    t68365
}
