//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 731/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk731<F: Float>(t5089: F, t555: F, t12: F, t137: F, t1643: F, t439: F) -> (F, F, F) {
    let t5091 = F::cast_from(0.10389515463408878255e3_f64) * t555 * t5089;
    let t5093 = F::cast_from(1.0_f64) / t137 / t12;
    let t5094 = t1643 * t439;
    (t5091, t5093, t5094)
}
