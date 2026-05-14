//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1067/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1067<F: Float>(t4038: F, t7179: F, t8152: F, t7174: F, t7213: F, t2433: F, t870: F, t981: F, t2360: F, t7294: F, t23817: F, t2529: F, t837: F, t845: F, t23789: F, t7341: F) -> (F, F, F, F, F, F) {
    let t24187 = t4038 * t8152 * t7179;
    let t24189 = t7213 * t7174;
    let t24190 = t2433 * t24189;
    let t24192 = t981 * t870;
    let t24197 = t2360 * t7294;
    let t24202 = 0.35089340384731224426e1 * t845 * t2529 * t23817 * t837;
    let t24206 = 0.1403573615389248977e2 * t845 * t7341 * t23789 * t837;
    (t24187, t24190, t24192, t24197, t24202, t24206)
}
