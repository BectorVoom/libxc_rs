//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 719/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk719<F: Float>(t3228: F, t542: F, t1008: F, t1588: F, t435: F, t4838: F, t386: F, t387: F, t174: F, t4099: F, t422: F, t537: F) -> (F, F, F, F, F, F) {
    let t5226 = t3228 * t542;
    let t5229 = F::new(0.85748036236139473944e-3) * t1008 * t1588;
    let t5230 = t435 * t4838;
    let t5232 = t386 * t387 * t5230;
    let t5235 = t174 * t4099;
    let t5237 = t422 * t387 * t5235;
    let t5240 = t3228 * t537;
    (t5226, t5229, t5232, t5235, t5237, t5240)
}
