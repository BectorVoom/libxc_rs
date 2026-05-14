//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 833/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk833<F: Float>(t5146: F, t555: F, t1511: F, t1639: F, t4911: F, t4913: F, t4915: F, t114: F, t1661: F, t557: F, t1508: F, t1675: F, t191: F, t545: F, t83: F, t1545: F, t546: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5148 = 0.35089341735807877242e1 * t555 * t5146;
    let t5149 = t1511 * t1639;
    let t5152 = t4911 * t4913 * t4915;
    let t5154 = 0.10254018858216406658e4 * t555 * t5152;
    let t5155 = t1661 * t114;
    let t5156 = t5155 * t557;
    let t5158 = t1511 * t1508;
    let t5165 = 1.0 / t1675 / t191;
    let t5169 = t1661 * t545;
    let t5170 = t83 * t5169;
    let t5177 = t1545 * t546;
    (t5148, t5149, t5152, t5154, t5155, t5156, t5158, t5165, t5169, t5170, t5177)
}
