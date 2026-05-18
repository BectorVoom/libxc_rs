//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 149/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk149<F: Float>(t106: F, t317: F, t335: F, t214: F, t226: F) -> (F, F, F) {
    let t339 = F::new(1.0) + F::new(0.27818116767324025134e1) * t106 * t317 * t335;
    let t340 = f64::ln(t339);
    let t346 = F::new(0.2568e1) + F::new(0.58165e1) * t214 + F::new(0.184725e-2) * t226;
    (t339, t340, t346)
}
