//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1296/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1296(t1165: f64, t3361: f64, t4267: f64, t4718: f64, t1173: f64, t14346: f64, t1552: f64, t18655: f64, t18657: f64, t18660: f64, t18672: f64, t18683: f64, t18686: f64, t20595: f64, t301: f64, t3403: f64, t4298: f64, t4463: f64, t4752: f64, t530: f64, t5621: f64, t6151: f64) -> f64 {
    let t24084 = t3361 * t1165 * t4267 * t4718;
    let t24104 = -0.40015750243531754508e-2_f64 * t18655 - 0.17149607247227894789e-2_f64 * t18657 + 0.42874018118069736972e-3_f64 * t14346 + 0.34299214494455789578e-2_f64 * t18660 + 0.51448821741683684367e-2_f64 * t18672 - 0.51448821741683684367e-2_f64 * t18683 - 0.68598428988911579156e-2_f64 * t24084 - 0.34299214494455789578e-1_f64 * t4463 * t1165 * t4267 * t4752 + 0.16006300097412701803e0_f64 * t18686 - 0.68598428988911579156e-2_f64 * t1173 * t1165 * t4298 * t5621 - 0.17149607247227894789e-1_f64 * t3403 * t1165 * t530 * t20595 - 0.68598428988911579156e-2_f64 * t1173 * t1165 * t1552 * t6151 * t301;
    t24104
}
