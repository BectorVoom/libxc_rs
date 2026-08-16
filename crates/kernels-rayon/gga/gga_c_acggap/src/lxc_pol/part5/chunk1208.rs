//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1208/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1208(t1439: f64, t322: f64, t13298: f64, t13299: f64, t525: f64, t1089: f64, t1180: f64, t13399: f64, t13400: f64, t16946: f64, t16950: f64, t22021: f64, t22023: f64, t22032: f64, t22038: f64, t3396: f64, t418: f64, t4680: f64, t5111: f64, t535: f64, t5795: f64, t5931: f64) -> (f64, f64) {
    let t22040 = t1439 * t322;
    let t22043 = t13298 * t13299 * t525 * t22040;
    let t22046 = -0.17149607247227894789e-2_f64 * t1180 * t4680 * t5795 + 0.13719685797782315831e-1_f64 * t3396 * t4680 * t5931 + 0.68598428988911579156e-2_f64 * t22021 + 0.17149607247227894789e-2_f64 * t22023 - 0.68598428988911579156e-2_f64 * t418 * t1089 * t535 * t5111 - 0.42874018118069736972e-2_f64 * t22032 - 0.17149607247227894789e-1_f64 * t16946 - 0.85748036236139473945e-2_f64 * t16950 - 0.68598428988911579156e-2_f64 * t22038 + 0.68598428988911579156e-2_f64 * t22043 + t13399 + 0.25724410870841842183e-2_f64 * t13400;
    (t22040, t22046)
}
