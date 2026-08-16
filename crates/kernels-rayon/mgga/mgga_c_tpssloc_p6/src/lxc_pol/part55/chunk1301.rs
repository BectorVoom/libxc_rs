//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1301/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1301(t118365: f64, t120809: f64, t120811: f64, t120818: f64, t120820: f64, t120823: f64, t120830: f64, t120835: f64, t120848: f64, t120851: f64, t123294: f64, t123296: f64, t123298: f64, t123306: f64, t125966: f64, t31287: f64, t32643: f64, t33192: f64, t4072: f64, t5376: f64, t577: f64) -> f64 {
    let t126015 = t120809 + t120811 + 54.0_f64 * t123294 + 54.0_f64 * t123296 + 0.45e1_f64 * t125966 * t577 + 54.0_f64 * t123298 + t120818 + 27.0_f64 * t118365 * t5376 + t120820 + t120823 + 27.0_f64 * t123306 + t120830 + t31287 + t120835 + 0.135e2_f64 * t32643 * t4072 + t33192 + t120848 + t120851;
    t126015
}
