//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 744/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk744<F: Float>(t1775: F, t3135: F, t3128: F, t11034: F, t3127: F, t2: F, t8275: F, t11008: F, t11013: F, t11017: F, t1787: F, t11059: F) -> (F, F, F, F, F, F, F) {
    let t11684 = F::new(4.0) / F::new(9.0) * t1775 * t3135;
    let t11686 = F::new(4.0) / F::new(27.0) * t1775 * t3128;
    let t11687 = t3127 * t11034;
    let t11690 = t8275 * t2;
    let t11691 = t11690 * t11008;
    let t11694 = t3127 * t11013;
    let t11697 = t1787 * t11017;
    let t11700 = t3127 * t11059;
    (t11684, t11686, t11687, t11691, t11694, t11697, t11700)
}
