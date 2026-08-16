//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3213/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3213<F: Float>(t19270: F, t3633: F, t4700: F, t63765: F, t63767: F, t63769: F, t63771: F, t63829: F, t64100: F, t64253: F, t64259: F, t64433: F, t65290: F, t65293: F, t65296: F, t65299: F) -> F {
    let t66891 = F::cast_from(2.0_f64) * t19270 * t3633 * t4700 + t63765 - t63767 + t63769 + t63771 - t63829 + t64100 + t64253 - t64259 + t64433 - t65290 - t65293 + t65296 - t65299;
    t66891
}
