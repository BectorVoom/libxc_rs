//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 886/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk886<F: Float>(t6548: F, t8511: F, t894: F, t3145: F, t8414: F, t2586: F, t3152: F, t1133: F, t1124: F, t3137: F, t1121: F, t1111: F, t3103: F, t3116: F, t3132: F, t4386: F, t8449: F, t8452: F, t8456: F, t8462: F, t8466: F, t8472: F, t8476: F, t8480: F, t8484: F, t8490: F, t8494: F, t8499: F, t8503: F, t8506: F, t8509: F) -> (F, F, F, F, F, F, F) {
    let t8512 = t8511 * t6548;
    let t8513 = t894 * t8512;
    let t8516 = t3145 * t8414;
    let t8517 = t8516 * t6548;
    let t8518 = t894 * t8517;
    let t8521 = t2586 * t3152;
    let t8522 = t1133 * t8521;
    let t8524 = t3137 * t1124;
    let t8525 = t1121 * t8524;
    let t8527 = F::cast_from(0.94667510637550784468e-1_f64) * t8449 + F::cast_from(0.71000632978163088351e-1_f64) * t3116 * t8452 + F::cast_from(0.71000632978163088351e-1_f64) * t3116 * t8456 + F::cast_from(0.11833438829693848058e0_f64) * t3116 * t8462 - F::cast_from(0.13735917720689745254e2_f64) * t3132 * t8466 + F::cast_from(0.27471835441379490507e2_f64) * t3103 * t8472 - F::cast_from(0.1420012659563261767e0_f64) * t3116 * t8476 + t8480 / F::new(216.0) + F::new(7.0) / F::new(648.0) * t1111 * t8484 + F::cast_from(0.18314556960919660338e2_f64) * t8490 - F::cast_from(0.10866451862235947318e-1_f64) * t4386 * t8494 + F::cast_from(0.90553765518632894319e-2_f64) * t4386 * t8499 - F::cast_from(0.91572784804598301689e1_f64) * t8503 - F::cast_from(0.12073835402484385909e-2_f64) * t8506 + F::cast_from(0.36221506207453157727e-2_f64) * t8509 + F::cast_from(0.10866451862235947318e-1_f64) * t1133 * t8513 - F::cast_from(0.18110753103726578864e-1_f64) * t1133 * t8518 - F::cast_from(0.72443012414906315455e-2_f64) * t8522 - F::cast_from(0.23666877659387696117e-1_f64) * t8525;
    (t8512, t8513, t8516, t8517, t8518, t8521, t8527)
}
