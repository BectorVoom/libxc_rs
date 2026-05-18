//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1097/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1097<F: Float>(t1967: F, t8838: F, t4352: F, t535: F, t598: F, t7656: F, t1089: F, t12473: F, t2288: F, t4360: F, t7741: F, t13287: F, t34823: F, t34828: F) -> (F, F, F, F, F) {
    let t35515 = t1967 * t8838;
    let t35519 = t598 * t4352 * t535 * t7656;
    let t35523 = t598 * t1089 * t12473 * t2288;
    let t35529 = t7741 * t4360;
    let t35535 = t34823 * t13287 * t34828;
    (t35515, t35519, t35523, t35529, t35535)
}
