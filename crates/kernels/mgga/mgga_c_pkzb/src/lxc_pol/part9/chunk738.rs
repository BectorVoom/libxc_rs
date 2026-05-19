//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 738/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk738<F: Float>(t5143: F, t1503: F, t4913: F, t541: F, t555: F, t1511: F, t1639: F, t4911: F, t4915: F, t114: F, t1661: F, t557: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5144 = F::new(144.0) * t5143;
    let t5146 = t1503 * t4913 * t541;
    let t5148 = F::cast_from(0.35089341735807877242e1_f64) * t555 * t5146;
    let t5149 = t1511 * t1639;
    let t5150 = F::cast_from(0.35089341735807877242e1_f64) * t5149;
    let t5152 = t4911 * t4913 * t4915;
    let t5154 = F::cast_from(0.10254018858216406658e4_f64) * t555 * t5152;
    let t5155 = t1661 * t114;
    let t5156 = t5155 * t557;
    (t5144, t5146, t5148, t5149, t5150, t5152, t5154, t5155, t5156)
}
