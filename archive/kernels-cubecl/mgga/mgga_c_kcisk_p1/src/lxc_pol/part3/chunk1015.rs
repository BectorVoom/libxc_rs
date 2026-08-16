//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1015/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1015<F: Float>(t12830: F, t4391: F, t1312: F, t12951: F, t539: F, t3952: F, t14966: F, t14973: F, t14980: F, t14983: F, t1568: F, t1572: F, t1580: F, t1593: F, t4378: F, t4381: F, t4397: F, t4403: F, t4408: F, t4499: F, t4502: F, t535: F) -> F {
    let t14991 = t4391 * t12830;
    let t14992 = t1312 * t14991;
    let t14995 = t539 * t12951;
    let t14996 = t14995 * t12830;
    let t14997 = t3952 * t14996;
    let t15004 = -F::cast_from(0.16191709844559585492e0_f64) * t535 * t14966 - F::cast_from(0.43177892918825561313e0_f64) * t1572 * t4378 + F::cast_from(0.53972366148531951639e-1_f64) * t1580 * t14973 + F::cast_from(0.14392630972941853771e0_f64) * t4381 * t4403 + F::cast_from(0.53972366148531951639e-1_f64) * t1580 * t14980 + F::cast_from(0.14392630972941853771e0_f64) * t14983 + F::cast_from(0.43177892918825561313e0_f64) * t4502 * t1593 + F::cast_from(0.21588946459412780656e0_f64) * t1572 * t4499 - F::cast_from(0.8095854922279792746e-1_f64) * t1568 * t4499 + F::cast_from(0.53972366148531951639e-1_f64) * t1580 * t14992 - F::cast_from(0.71963154864709268855e-1_f64) * t1580 * t14997 - F::cast_from(0.53972366148531951639e-1_f64) * t4397 * t4408 + F::cast_from(0.1439263097294185377e0_f64) * t4381 * t4408;
    t15004
}
