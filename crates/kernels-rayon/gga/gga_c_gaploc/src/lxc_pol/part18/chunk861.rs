//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 861/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk861(t1: f64, t8519: f64, t787: f64, t1966: f64, t2028: f64, t2087: f64, t2103: f64, t2197: f64, t2639: f64, t3011: f64, t3015: f64, t3022: f64, t5771: f64, t5782: f64, t7432: f64, t7436: f64, t7504: f64, t807: f64, t825: f64, t833: f64, t8471: f64, t8475: f64, t8478: f64, t8485: f64, t8489: f64, t8494: f64, t8497: f64, t8504: f64, t8509: f64, t8513: f64, t8516: f64) -> (f64, f64, f64) {
    let t8520 = t8519 * t1;
    let t8521 = t787 * t8520;
    let t8524 = 0.23005755572352449806e2_f64 * t2197 * t3022 + 0.23005755572352449806e2_f64 * t833 * t8471 + 0.11502877786176224903e2_f64 * t833 * t8475 - 0.21450293971110256002e1_f64 * t8478 * t2639 - 0.13803453343411469884e2_f64 * t5782 * t3011 - 0.13803453343411469884e2_f64 * t2087 * t8485 - 0.69017266717057349418e1_f64 * t2087 * t8489 + 0.14300195980740170668e1_f64 * t5771 * t3015 + 0.14300195980740170668e1_f64 * t2103 * t8494 + 0.71500979903700853338e0_f64 * t2103 * t8497 - 0.59584149919750711116e-1_f64 * t7432 - 0.29792074959875355558e-1_f64 * t7436 - 0.51123901271894332902e1_f64 * t1966 * t8504 + 0.17875244975925213335e0_f64 * t7504 - 0.11360866949309851756e0_f64 * t825 * t8509 + 0.23005755572352449806e1_f64 * t807 * t8513 - 0.79445533226334281486e-1_f64 * t8516 * t2028 - 0.79445533226334281486e-1_f64 * t8521 * t2028;
    (t8520, t8521, t8524)
}
