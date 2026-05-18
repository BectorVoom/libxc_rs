//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 862/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk862<F: Float>(t1: F, t8519: F, t787: F, t1966: F, t2028: F, t2087: F, t2103: F, t2197: F, t2639: F, t3011: F, t3015: F, t3022: F, t5771: F, t5782: F, t7432: F, t7436: F, t7504: F, t807: F, t825: F, t833: F, t8471: F, t8475: F, t8478: F, t8485: F, t8489: F, t8494: F, t8497: F, t8504: F, t8509: F, t8513: F, t8516: F) -> (F, F, F) {
    let t8520 = t8519 * t1;
    let t8521 = t787 * t8520;
    let t8524 = F::new(0.23005755572352449806e2) * t2197 * t3022 + F::new(0.23005755572352449806e2) * t833 * t8471 + F::new(0.11502877786176224903e2) * t833 * t8475 - F::new(0.21450293971110256002e1) * t8478 * t2639 - F::new(0.13803453343411469884e2) * t5782 * t3011 - F::new(0.13803453343411469884e2) * t2087 * t8485 - F::new(0.69017266717057349418e1) * t2087 * t8489 + F::new(0.14300195980740170668e1) * t5771 * t3015 + F::new(0.14300195980740170668e1) * t2103 * t8494 + F::new(0.71500979903700853338e0) * t2103 * t8497 - F::new(0.59584149919750711116e-1) * t7432 - F::new(0.29792074959875355558e-1) * t7436 - F::new(0.51123901271894332902e1) * t1966 * t8504 + F::new(0.17875244975925213335e0) * t7504 - F::new(0.11360866949309851756e0) * t825 * t8509 + F::new(0.23005755572352449806e1) * t807 * t8513 - F::new(0.79445533226334281486e-1) * t8516 * t2028 - F::new(0.79445533226334281486e-1) * t8521 * t2028;
    (t8520, t8521, t8524)
}
