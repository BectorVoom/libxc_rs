//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1015/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1015(t12830: f64, t4391: f64, t1312: f64, t12951: f64, t539: f64, t3952: f64, t14966: f64, t14973: f64, t14980: f64, t14983: f64, t1568: f64, t1572: f64, t1580: f64, t1593: f64, t4378: f64, t4381: f64, t4397: f64, t4403: f64, t4408: f64, t4499: f64, t4502: f64, t535: f64) -> f64 {
    let t14991 = t4391 * t12830;
    let t14992 = t1312 * t14991;
    let t14995 = t539 * t12951;
    let t14996 = t14995 * t12830;
    let t14997 = t3952 * t14996;
    let t15004 = -0.16191709844559585492e0_f64 * t535 * t14966 - 0.43177892918825561313e0_f64 * t1572 * t4378 + 0.53972366148531951639e-1_f64 * t1580 * t14973 + 0.14392630972941853771e0_f64 * t4381 * t4403 + 0.53972366148531951639e-1_f64 * t1580 * t14980 + 0.14392630972941853771e0_f64 * t14983 + 0.43177892918825561313e0_f64 * t4502 * t1593 + 0.21588946459412780656e0_f64 * t1572 * t4499 - 0.8095854922279792746e-1_f64 * t1568 * t4499 + 0.53972366148531951639e-1_f64 * t1580 * t14992 - 0.71963154864709268855e-1_f64 * t1580 * t14997 - 0.53972366148531951639e-1_f64 * t4397 * t4408 + 0.1439263097294185377e0_f64 * t4381 * t4408;
    t15004
}
