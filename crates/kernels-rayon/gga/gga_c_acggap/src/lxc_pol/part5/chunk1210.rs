//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1210/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1210(t1032: f64, t6194: f64, t6071: f64, t935: f64, t1017: f64, t1165: f64, t1173: f64, t1180: f64, t1181: f64, t1426: f64, t16980: f64, t16988: f64, t16990: f64, t1743: f64, t1748: f64, t1772: f64, t22048: f64, t22068: f64, t22080: f64, t368: f64, t397: f64, t398: f64, t418: f64, t4289: f64, t4298: f64, t4313: f64, t530: f64, t5616: f64, t6394: f64, t922: f64, t955: f64) -> f64 {
    let t22082 = t1032 * t6194;
    let t22085 = t935 * t6071;
    let t22091 = 0.34299214494455789578e-2_f64 * t1173 * t1181 * t530 * t22048 + 0.34299214494455789578e-2_f64 * t1180 * t1181 * t4298 * t6394 - 0.17149607247227894789e-2_f64 * t1180 * t1181 * t4289 * t5616 + 0.10289764348336736873e-1_f64 * t1173 * t1165 * t4313 * t1748 * t1017 + 0.42874018118069736972e-3_f64 * t22068 - 0.17149607247227894789e-2_f64 * t16980 - 0.25724410870841842183e-2_f64 * t16988 + 0.85748036236139473945e-2_f64 * t418 * t1426 * t368 * t1772 * t922 + 0.85748036236139473944e-3_f64 * t22080 + 0.40015750243531754508e-2_f64 * t22082 + 0.48018900292238105408e-1_f64 * t16990 - 0.42874018118069736972e-3_f64 * t22085 - 0.42874018118069736972e-3_f64 * t397 * t398 * t1743 * t955;
    t22091
}
