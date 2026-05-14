//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 567/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk567<F: Float>(t1096: F, t1165: F, t4417: F, t1466: F, t3409: F, t1106: F, t1181: F, t540: F, t3391: F, t1131: F, t336: F, t535: F, t1198: F, t513: F, t157: F, t1552: F) -> (F, F, F, F, F, F, F) {
    let t4419 = t1165 * t4417 * t1096;
    let t4423 = 0.40015750243531754508e-2 * t3409 * t1466;
    let t4425 = t1181 * t540 * t1106;
    let t4427 = 0.17149607247227894789e-2 * t3391 * t4425;
    let t4430 = t336 * t535 * t1131;
    let t4434 = t336 * t1198 * t513;
    let t4437 = t157 * t1131;
    let t4439 = t1165 * t1552 * t4437;
    (t4419, t4423, t4425, t4427, t4430, t4434, t4439)
}
