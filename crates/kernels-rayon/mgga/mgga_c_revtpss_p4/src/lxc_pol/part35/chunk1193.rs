//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1193/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1193(t114: f64, t1450: f64, t22809: f64, t1907: f64, t6922: f64, t1868: f64, t6781: f64, t22633: f64, t94: f64, t6816: f64, t101451: f64, t105870: f64, t105878: f64, t114394: f64, t114396: f64, t114398: f64, t95397: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t114776 = t1450 * t22809;
    let t114780 = t1907 * t6922;
    let t114791 = t1868 * t6781;
    let t114800 = t1868 * t6922;
    let t114812 = t94 * t22633;
    let t114820 = t6816 * t1907;
    let t114905 = piecewise3(t115, 0.0_f64, -t95397 - 22.0_f64 / 3.0_f64 * t101451 - 4.0_f64 * t105870 + 2.0_f64 * t105878 - 3.0_f64 / 2.0_f64 * t114394 + 3.0_f64 / 2.0_f64 * t114396 - t114398 / 4.0_f64);
    (t114776, t114780, t114791, t114800, t114812, t114820, t114905)
}
