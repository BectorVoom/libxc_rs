//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 595/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk595<F: Float>(t1096: F, t1165: F, t4417: F, t1466: F, t3409: F, t1106: F, t1181: F, t540: F, t3391: F, t1131: F, t336: F, t535: F) -> (F, F, F, F, F) {
    let t4419 = t1165 * t4417 * t1096;
    let t4423 = F::new(0.40015750243531754508e-2) * t3409 * t1466;
    let t4425 = t1181 * t540 * t1106;
    let t4427 = F::new(0.17149607247227894789e-2) * t3391 * t4425;
    let t4430 = t336 * t535 * t1131;
    (t4419, t4423, t4425, t4427, t4430)
}
