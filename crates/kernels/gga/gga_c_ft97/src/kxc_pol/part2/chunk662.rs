//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 662/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk662<F: Float>(t1771: F, t963: F, t358: F, t378: F, t93: F, t1587: F, t1755: F, t3149: F, t1775: F, t3135: F, t3128: F, t11034: F, t3127: F, t2: F, t8275: F, t11008: F) -> (F, F, F, F, F, F, F) {
    let t11669 = t1771 * t963;
    let t11672 = t378 * t93 * t358;
    let t11676 = t1587 * t3149 * t1755;
    let t11684 = 4.0 / 9.0 * t1775 * t3135;
    let t11686 = 4.0 / 27.0 * t1775 * t3128;
    let t11687 = t3127 * t11034;
    let t11690 = t8275 * t2;
    let t11691 = t11690 * t11008;
    (t11669, t11672, t11676, t11684, t11686, t11687, t11691)
}
