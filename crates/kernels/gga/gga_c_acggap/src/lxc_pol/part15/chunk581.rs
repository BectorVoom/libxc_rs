//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 581/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk581<F: Float>(t1089: F, t175: F, t5249: F, t384: F, t1032: F, t1423: F, t1539: F, t301: F, t1165: F, t1532: F, t3194: F, t1647: F, t879: F) -> (F, F, F, F, F, F, F) {
    let t5251 = t1089 * t175 * t5249;
    let t5253 = F::new(0.17149607247227894789e-2) * t384 * t5251;
    let t5263 = t1032 * t1423;
    let t5284 = t1539 * t301;
    let t5286 = t1165 * t1532 * t5284;
    let t5288 = F::new(0.17149607247227894789e-2) * t3194 * t5286;
    let t5304 = t1647 * t879;
    (t5251, t5253, t5263, t5284, t5286, t5288, t5304)
}
