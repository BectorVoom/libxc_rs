//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1285/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1285(t13298: f64, t13299: f64, t1849: f64, t4210: f64, t12572: f64, t6376: f64, t1137: f64, t6297: f64, t4389: f64, t5755: f64, t1165: f64, t1173: f64, t13591: f64, t14221: f64, t14228: f64, t14233: f64, t14239: f64, t14242: f64, t18426: f64, t301: f64, t5275: f64, t5852: f64, t5853: f64, t5867: f64) -> f64 {
    let t23781 = t13298 * t13299 * t1849 * t4210;
    let t23787 = t12572 * t6376;
    let t23789 = t1137 * t6297;
    let t23792 = t4389 * t5755;
    let t23803 = 0.68598428988911579156e-2_f64 * t23781 - 0.16006300097412701803e-1_f64 * t14221 - 0.42874018118069736972e-3_f64 * t14228 - 0.42874018118069736972e-3_f64 * t14233 + 0.42874018118069736972e-3_f64 * t14239 + t14242 + 7.0_f64 / 6.0_f64 * t23787 - 7.0_f64 / 36.0_f64 * t23789 + 0.16006300097412701803e-1_f64 * t18426 - 0.16006300097412701803e-1_f64 * t23792 - 0.34299214494455789578e-2_f64 * t1173 * t1165 * t5867 * t5275 + 0.10289764348336736874e-1_f64 * t13591 * t1165 * t5852 * t5853 * t301;
    t23803
}
