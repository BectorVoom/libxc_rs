//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 833/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk833<F: Float>(t420: F, t5578: F, t1608: F, t32167: F, t32237: F, t32240: F, t378: F, t32156: F, t66: F, t1669: F, t32168: F, t173: F, t32260: F, t22819: F, t7195: F, t1293: F, t37: F) -> (F, F, F, F, F, F, F, F) {
    let t136469 = t5578 * t420;
    let t136474 = t1608 * t32167 * t32237;
    let t136475 = t32240 * t378;
    let t136485 = t32156 * t66;
    let t136488 = t1669 * t32168;
    let t136505 = t173 * t32260;
    let t136507 = t22819 * t7195 * t136505;
    let t136516 = t37 * t1293;
    (t136469, t136474, t136475, t136485, t136488, t136505, t136507, t136516)
}
