//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1355/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1355<F: Float>(t1128: F, t8913: F, t8917: F, t1122: F, t1133: F, t11943: F, t12029: F, t26317: F, t26322: F, t26880: F, t26954: F, t26977: F, t26981: F, t26987: F, t26991: F, t3106: F, t3116: F, t3117: F, t3119: F, t3120: F, t3126: F, t3235: F, t4355: F, t4374: F, t4386: F, t4387: F, t6554: F, t8459: F, t8461: F, t8465: F, t8905: F, t8966: F, t8969: F, t9128: F) -> F {
    let t26995 = t8913 * t1128 * t8917;
    let t26997 = -F::cast_from(0.63777043459628018516e5_f64) * t9128 * t8465 * t12029 + F::cast_from(0.1062950724327133642e5_f64) * t11943 * t3106 * t26954 - F::cast_from(0.14488602482981263091e-1_f64) * t4386 * t3235 * t26317 + F::cast_from(0.12073835402484385909e-1_f64) * t4386 * t4387 * t26322 + F::cast_from(0.94667510637550784468e-1_f64) * t3116 * t4374 * t6554 * t1122 * t3119 + F::cast_from(0.94667510637550784468e-1_f64) * t3116 * t3117 * t8905 * t3120 + F::cast_from(0.23666877659387696117e0_f64) * t3116 * t8459 * t3126 * t8461 - F::cast_from(0.28345352648723563785e5_f64) * t9128 * t26880 * t26977 - F::cast_from(0.36629113921839320676e2_f64) * t8966 * t26981 * t4355 * t8969 + F::cast_from(0.94667510637550784468e-1_f64) * t26987 + F::cast_from(0.10866451862235947318e0_f64) * t1133 * t26991 + F::cast_from(0.28345352648723563784e5_f64) * t26995;
    t26997
}
