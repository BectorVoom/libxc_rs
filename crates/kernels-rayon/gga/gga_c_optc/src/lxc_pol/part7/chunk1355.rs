//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1355/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1355(t1128: f64, t8913: f64, t8917: f64, t1122: f64, t1133: f64, t11943: f64, t12029: f64, t26317: f64, t26322: f64, t26880: f64, t26954: f64, t26977: f64, t26981: f64, t26987: f64, t26991: f64, t3106: f64, t3116: f64, t3117: f64, t3119: f64, t3120: f64, t3126: f64, t3235: f64, t4355: f64, t4374: f64, t4386: f64, t4387: f64, t6554: f64, t8459: f64, t8461: f64, t8465: f64, t8905: f64, t8966: f64, t8969: f64, t9128: f64) -> f64 {
    let t26995 = t8913 * t1128 * t8917;
    let t26997 = -0.63777043459628018516e5_f64 * t9128 * t8465 * t12029 + 0.1062950724327133642e5_f64 * t11943 * t3106 * t26954 - 0.14488602482981263091e-1_f64 * t4386 * t3235 * t26317 + 0.12073835402484385909e-1_f64 * t4386 * t4387 * t26322 + 0.94667510637550784468e-1_f64 * t3116 * t4374 * t6554 * t1122 * t3119 + 0.94667510637550784468e-1_f64 * t3116 * t3117 * t8905 * t3120 + 0.23666877659387696117e0_f64 * t3116 * t8459 * t3126 * t8461 - 0.28345352648723563785e5_f64 * t9128 * t26880 * t26977 - 0.36629113921839320676e2_f64 * t8966 * t26981 * t4355 * t8969 + 0.94667510637550784468e-1_f64 * t26987 + 0.10866451862235947318e0_f64 * t1133 * t26991 + 0.28345352648723563784e5_f64 * t26995;
    t26997
}
