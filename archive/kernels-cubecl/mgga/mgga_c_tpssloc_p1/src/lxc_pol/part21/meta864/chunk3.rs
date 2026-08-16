//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3154/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3154<F: Float>(t63714: F, t63717: F, t63720: F, t63722: F, t63725: F, t63729: F, t64536: F, t64540: F, t64558: F, t64562: F, t64564: F, t64566: F) -> F {
    let t65285 = t64536 + t64540 - t64558 + t64562 - t64564 - t64566 + t63714 + t63717 + t63720 + t63722 + t63725 + t63729;
    t65285
}
