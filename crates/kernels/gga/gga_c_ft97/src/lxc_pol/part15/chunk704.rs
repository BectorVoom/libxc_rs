//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 704/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk704<F: Float>(t1127: F, t4978: F, t680: F, t17964: F, t17993: F, t21277: F, t21282: F, t21285: F, t21289: F, t21292: F, t21297: F, t21300: F, t21306: F, t21309: F, t2379: F, t2387: F, t2394: F, t3759: F, t6757: F, t678: F, t807: F, t9524: F, t9609: F) -> (F, F) {
    let t21319 = t4978 * t1127;
    let t21320 = t680 * t21319;
    let t21323 = -0.69764702839313376e-1 * t3759 * t680 * t21277 - 0.69764702839313376e-1 * t3759 * t21282 + 0.11619434043764639964e-2 * t3759 * t2379 * t21285 - 0.58097170218823199822e-3 * t2387 * t21289 + 0.139529405678626752e0 * t17964 * t6757 * t21292 + 0.69764702839313376e-2 * t678 * t21297 + 0.58097170218823199823e-3 * t678 * t21300 + 0.139529405678626752e-1 * t3759 * t2394 * t21285 - 0.7112856777411015585e-1 * t17993 * t21306 - 0.32253953169881963531e-5 * t678 * t807 * t21309 - 0.279058811357253504e-2 * t678 * t9609 * t21309 - 0.11619434043764639964e-3 * t678 * t9524 * t21309 + 0.34882351419656688e-1 * t2387 * t21320;
    (t21319, t21323)
}
