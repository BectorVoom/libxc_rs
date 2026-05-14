//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 498/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk498<F: Float>(t1786: F, t971: F, t463: F, t3539: F, t604: F, t135: F, t3347: F, t131: F, t538: F, t71: F, t929: F, t1045: F, t2178: F) -> (F, F, F, F, F, F, F) {
    let t11902 = t1786 * t971;
    let t11906 = t463 * t971;
    let t12277 = t3539 * t604;
    let t12374 = t3347 * t135;
    let t12411 = t538 * t131;
    let t12486 = t71 * t929;
    let t12664 = t1045 * t2178;
    (t11902, t11906, t12277, t12374, t12411, t12486, t12664)
}
