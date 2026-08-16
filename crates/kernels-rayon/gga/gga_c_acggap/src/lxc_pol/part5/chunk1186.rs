//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1186/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1186(t3621: f64, t5916: f64, t1137: f64, t5919: f64, t1084: f64, t1090: f64, t1181: f64, t16674: f64, t16676: f64, t16678: f64, t16680: f64, t16686: f64, t16688: f64, t1879: f64, t20545: f64, t3396: f64, t367: f64, t4479: f64, t4593: f64, t4735: f64, t5187: f64) -> f64 {
    let t21557 = t3621 * t5916;
    let t21559 = t1137 * t5919;
    let t21575 = t367 * t4593 * t5187 / 12.0_f64 + t367 * t4593 * t4479 / 24.0_f64 - 7.0_f64 / 12.0_f64 * t21557 - 7.0_f64 / 36.0_f64 * t21559 - 0.20579528696673473748e-1_f64 * t3396 * t1181 * t1879 * t1090 - 0.20579528696673473748e-1_f64 * t4735 * t1181 * t20545 * t1084 + 0.24009450146119052704e-1_f64 * t16674 + 0.45351183609335988442e-1_f64 * t16676 - 0.45351183609335988442e-1_f64 * t16678 - 0.17149607247227894789e-2_f64 * t16680 + 0.68026775414003982663e-1_f64 * t16686 + 0.45351183609335988442e-1_f64 * t16688;
    t21575
}
