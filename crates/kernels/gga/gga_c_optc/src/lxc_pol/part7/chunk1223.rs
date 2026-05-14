//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1223/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1223<F: Float>(t26881: F, t3126: F, t1028: F, t9123: F, t1135: F, t1121: F, t1128: F, t8907: F, t3145: F, t8428: F, t22035: F, t894: F, t8913: F, t8917: F, t1122: F, t1133: F, t11943: F, t12029: F, t26317: F, t26322: F, t26880: F, t3106: F, t3116: F, t3117: F, t3119: F, t3120: F, t3235: F, t4355: F, t4374: F, t4386: F, t4387: F, t6554: F, t8459: F, t8461: F, t8465: F, t8905: F, t8966: F, t8969: F, t9128: F) -> (F, F, F) {
    let t26954 = t26881 * t3126;
    let t26977 = t9123 * t1028;
    let t26981 = t1135 * t3126;
    let t26987 = t1121 * t1128 * t8907;
    let t26989 = t3145 * t8428;
    let t26991 = t894 * t26989 * t22035;
    let t26995 = t8913 * t1128 * t8917;
    let t26997 = -0.63777043459628018516e5 * t9128 * t8465 * t12029 + 0.1062950724327133642e5 * t11943 * t3106 * t26954 - 0.14488602482981263091e-1 * t4386 * t3235 * t26317 + 0.12073835402484385909e-1 * t4386 * t4387 * t26322 + 0.94667510637550784468e-1 * t3116 * t4374 * t6554 * t1122 * t3119 + 0.94667510637550784468e-1 * t3116 * t3117 * t8905 * t3120 + 0.23666877659387696117e0 * t3116 * t8459 * t3126 * t8461 - 0.28345352648723563785e5 * t9128 * t26880 * t26977 - 0.36629113921839320676e2 * t8966 * t26981 * t4355 * t8969 + 0.94667510637550784468e-1 * t26987 + 0.10866451862235947318e0 * t1133 * t26991 + 0.28345352648723563784e5 * t26995;
    (t26981, t26991, t26997)
}
