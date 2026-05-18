//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 474/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk474<F: Float>(t2329: F, t278: F, t2328: F, t2326: F, t136: F, t3: F, t362: F, t190: F, t288: F) -> (F, F, F, F, F) {
    let t2330 = t2329 * t278;
    let t2331 = F::new(1.0) / t2330;
    let t2332 = t2328 * t2331;
    let t2333 = t2326 * t2332;
    let t2335 = t136 * t3;
    let t2336 = t2335 * t362;
    let t2337 = t288 * t190 * t2336;
    (t2331, t2332, t2333, t2336, t2337)
}
