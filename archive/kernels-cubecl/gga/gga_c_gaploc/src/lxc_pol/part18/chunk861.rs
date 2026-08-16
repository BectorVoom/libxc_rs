//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 861/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk861<F: Float>(t1: F, t8519: F, t787: F, t1966: F, t2028: F, t2087: F, t2103: F, t2197: F, t2639: F, t3011: F, t3015: F, t3022: F, t5771: F, t5782: F, t7432: F, t7436: F, t7504: F, t807: F, t825: F, t833: F, t8471: F, t8475: F, t8478: F, t8485: F, t8489: F, t8494: F, t8497: F, t8504: F, t8509: F, t8513: F, t8516: F) -> (F, F, F) {
    let t8520 = t8519 * t1;
    let t8521 = t787 * t8520;
    let t8524 = F::cast_from(0.23005755572352449806e2_f64) * t2197 * t3022 + F::cast_from(0.23005755572352449806e2_f64) * t833 * t8471 + F::cast_from(0.11502877786176224903e2_f64) * t833 * t8475 - F::cast_from(0.21450293971110256002e1_f64) * t8478 * t2639 - F::cast_from(0.13803453343411469884e2_f64) * t5782 * t3011 - F::cast_from(0.13803453343411469884e2_f64) * t2087 * t8485 - F::cast_from(0.69017266717057349418e1_f64) * t2087 * t8489 + F::cast_from(0.14300195980740170668e1_f64) * t5771 * t3015 + F::cast_from(0.14300195980740170668e1_f64) * t2103 * t8494 + F::cast_from(0.71500979903700853338e0_f64) * t2103 * t8497 - F::cast_from(0.59584149919750711116e-1_f64) * t7432 - F::cast_from(0.29792074959875355558e-1_f64) * t7436 - F::cast_from(0.51123901271894332902e1_f64) * t1966 * t8504 + F::cast_from(0.17875244975925213335e0_f64) * t7504 - F::cast_from(0.11360866949309851756e0_f64) * t825 * t8509 + F::cast_from(0.23005755572352449806e1_f64) * t807 * t8513 - F::cast_from(0.79445533226334281486e-1_f64) * t8516 * t2028 - F::cast_from(0.79445533226334281486e-1_f64) * t8521 * t2028;
    (t8520, t8521, t8524)
}
