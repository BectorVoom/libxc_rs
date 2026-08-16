//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 886/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk886(t6548: f64, t8511: f64, t894: f64, t3145: f64, t8414: f64, t2586: f64, t3152: f64, t1133: f64, t1124: f64, t3137: f64, t1121: f64, t1111: f64, t3103: f64, t3116: f64, t3132: f64, t4386: f64, t8449: f64, t8452: f64, t8456: f64, t8462: f64, t8466: f64, t8472: f64, t8476: f64, t8480: f64, t8484: f64, t8490: f64, t8494: f64, t8499: f64, t8503: f64, t8506: f64, t8509: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8512 = t8511 * t6548;
    let t8513 = t894 * t8512;
    let t8516 = t3145 * t8414;
    let t8517 = t8516 * t6548;
    let t8518 = t894 * t8517;
    let t8521 = t2586 * t3152;
    let t8522 = t1133 * t8521;
    let t8524 = t3137 * t1124;
    let t8525 = t1121 * t8524;
    let t8527 = 0.94667510637550784468e-1_f64 * t8449 + 0.71000632978163088351e-1_f64 * t3116 * t8452 + 0.71000632978163088351e-1_f64 * t3116 * t8456 + 0.11833438829693848058e0_f64 * t3116 * t8462 - 0.13735917720689745254e2_f64 * t3132 * t8466 + 0.27471835441379490507e2_f64 * t3103 * t8472 - 0.1420012659563261767e0_f64 * t3116 * t8476 + t8480 / 216.0_f64 + 7.0_f64 / 648.0_f64 * t1111 * t8484 + 0.18314556960919660338e2_f64 * t8490 - 0.10866451862235947318e-1_f64 * t4386 * t8494 + 0.90553765518632894319e-2_f64 * t4386 * t8499 - 0.91572784804598301689e1_f64 * t8503 - 0.12073835402484385909e-2_f64 * t8506 + 0.36221506207453157727e-2_f64 * t8509 + 0.10866451862235947318e-1_f64 * t1133 * t8513 - 0.18110753103726578864e-1_f64 * t1133 * t8518 - 0.72443012414906315455e-2_f64 * t8522 - 0.23666877659387696117e-1_f64 * t8525;
    (t8512, t8513, t8516, t8517, t8518, t8521, t8527)
}
