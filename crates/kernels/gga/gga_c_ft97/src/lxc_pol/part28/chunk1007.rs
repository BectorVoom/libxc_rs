//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1007/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1007<F: Float>(t136015: F, t6418: F, t1286: F, t34588: F, t376: F, t25545: F, t7162: F, t25587: F, t26061: F, t5743: F, t137561: F, t979: F) -> (F, F, F, F, F, F) {
    let t144623 = t136015 * t6418;
    let t144633 = t1286 * t376 * t34588;
    let t144635 = t7162 * t25545;
    let t144641 = t7162 * t25587;
    let t144643 = t26061 * t5743;
    let t144645 = t137561 * t979;
    (t144623, t144633, t144635, t144641, t144643, t144645)
}
