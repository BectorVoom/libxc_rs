//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1057/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1057<F: Float>(t2360: F, t7275: F, t352: F, t8287: F, t8289: F, t8294: F, t870: F, t20: F, t2434: F, t362: F, t5: F, t770: F, t23: F, t2548: F, t191: F, t8386: F) -> (F, F, F, F) {
    let t23968 = t2360 * t7275;
    let t23970 = t8287 * t352;
    let t23973 = t23970 * t870 * t8289 * t8294;
    let t23974 = t2434 * t20;
    let t23975 = t5 * t362;
    let t23977 = t23974 * t23975 * t770;
    let t23982 = t23 * t2548;
    let t23983 = t23982 * t191;
    let t23984 = t23983 * t8386;
    (t23968, t23973, t23977, t23984)
}
