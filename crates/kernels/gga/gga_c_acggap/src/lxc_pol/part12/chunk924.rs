//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 924/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk924<F: Float>(t31346: F, t4269: F, t10098: F, t8462: F, t8653: F, t30407: F, t30408: F, t30409: F, t495: F, t30402: F, t506: F, t30418: F, t31102: F, t513: F, t7447: F, t8637: F) -> (F, F, F, F, F, F, F, F) {
    let t34563 = t31346 * t4269;
    let t34569 = t10098 * t8462;
    let t34570 = t34569 * t8653;
    let t34578 = t30407 * t30408 * t30409 * t495;
    let t34582 = t30407 * t30402 * t30409 * t506;
    let t34586 = t30407 * t30418 * t31102 * t513;
    let t34590 = t30407 * t30402 * t30409 * t513;
    let t34592 = t7447 * t8637;
    (t34563, t34569, t34570, t34578, t34582, t34586, t34590, t34592)
}
