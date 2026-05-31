//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3224/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3224<F: Float>(t2609: F, t2611: F, t5819: F, t49957: F, t49963: F, t49966: F, t49978: F, t49981: F, t49983: F, t49986: F, t39779: F, t39783: F, t39786: F, t39791: F, t39795: F) -> (F, F, F, F, F, F, F, F, F) {
    let t61165 = t2611 * t2609 * t5819;
    let t61166 = F::cast_from(12.0_f64) * t61165;
    let t61167 = F::cast_from(0.23392894490538584828e1_f64) * t49957;
    let t61168 = F::cast_from(0.69263436422725855034e2_f64) * t49963;
    let t61169 = F::cast_from(0.11696447245269292414e1_f64) * t49966;
    let t61170 = F::cast_from(8.0_f64) * t49978;
    let t61171 = F::cast_from(16.0_f64) * t49981;
    let t61172 = F::cast_from(8.0_f64) * t49983;
    let t61173 = F::cast_from(0.36622894612013090108e-3_f64) * t49986;
    let t61174 = t61166 - t61167 + t39779 - t61168 - t61169 - t39783 - t39786 - t39791 - t39795 + t61170 + t61171 + t61172 - t61173;
    (t61166, t61167, t61168, t61169, t61170, t61171, t61172, t61173, t61174)
}
