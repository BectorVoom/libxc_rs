//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 815/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk815<F: Float>(t5087: F, t8232: F, t5066: F, t2492: F, t5132: F, t5153: F, t222: F, t2382: F, t226: F, t17837: F, t4952: F, t13519: F, t5019: F, t17831: F, t3771: F, t9523: F) -> (F, F, F, F, F, F, F, F) {
    let t65437 = t8232 * t5087;
    let t65508 = t8232 * t5066;
    let t65592 = t2492 * t5132;
    let t65644 = t8232 * t5153;
    let t65692 = t2382 * t222;
    let t65693 = t65692 * t226;
    let t65695 = t17837 * t4952;
    let t65702 = t13519 * t5019;
    let t65735 = t3771 * t17831 * t9523;
    (t65437, t65508, t65592, t65644, t65693, t65695, t65702, t65735)
}
