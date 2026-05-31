//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 775/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk775<F: Float>(t7028: F, t83: F, t1508: F, t2609: F, t114: F, t2557: F, t557: F, t1499: F, t1639: F, t5177: F, t1008: F, t49: F) -> (F, F, F, F, F, F, F, F) {
    let t7030 = F::cast_from(2.0_f64) * t83 * t7028;
    let t7033 = t2609 * t1508;
    let t7035 = t2557 * t114;
    let t7037 = F::cast_from(0.11696447245269292414e1_f64) * t7035 * t557;
    let t7038 = t2609 * t1499;
    let t7040 = t2609 * t1639;
    let t7042 = F::cast_from(12.0_f64) * t5177;
    let t7046 = t1008 * t49;
    (t7030, t7033, t7035, t7037, t7038, t7040, t7042, t7046)
}
