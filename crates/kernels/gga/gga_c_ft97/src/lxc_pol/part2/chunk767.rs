//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 767/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk767<F: Float>(t3200: F, t8372: F, t3255: F, t487: F, t379: F, t1909: F, t3183: F, t8506: F, t11604: F, t3194: F, t3193: F, t1851: F, t358: F) -> (F, F, F, F, F) {
    let t12030 = t8372 * t3200;
    let t12033 = t487 * t3255;
    let t12034 = t12033 * t379;
    let t12035 = t1909 * t12034;
    let t12038 = t8506 * t3183;
    let t12041 = t3194 * t11604;
    let t12042 = t3193 * t12041;
    let t12045 = t1851 * t358;
    (t12030, t12035, t12038, t12042, t12045)
}
