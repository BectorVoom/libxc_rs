//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1118/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1118<F: Float>(t1111: F, t530: F, t5318: F, t3103: F, t3109: F, t46715: F, t3079: F, t5276: F, t1121: F, t3137: F, t5313: F, t2639: F, t5416: F) -> (F, F, F, F, F) {
    let t47001 = t1111 * t530 * t5318;
    let t47069 = t3103 * t46715 * t3109;
    let t47138 = t5276 * t3079;
    let t47149 = t1121 * t3137 * t5313;
    let t47155 = t5416 * t2639;
    (t47001, t47069, t47138, t47149, t47155)
}
