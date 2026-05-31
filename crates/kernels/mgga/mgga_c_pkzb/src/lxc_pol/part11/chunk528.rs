//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 528/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk528<F: Float>(t204: F, t205: F, t2739: F, t1831: F, t1833: F, t2730: F, t228: F, t1070: F, t663: F) -> (F, F, F, F) {
    let t2741 = t204 * t205 * t2739;
    let t2743 = t1831 - F::cast_from(0.17808333333333333333e-1_f64) * t1833 - F::cast_from(0.17808333333333333333e-1_f64) * t2730 + F::cast_from(0.53425e-1_f64) * t2741;
    let t2745 = F::cast_from(0.621814e-1_f64) * t2743 * t228;
    let t2746 = t1070 * t663;
    (t2741, t2743, t2745, t2746)
}
