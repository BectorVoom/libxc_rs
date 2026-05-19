//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 197/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk197<F: Float>(t626: F, t633: F, t184: F, t188: F, t622: F) -> (F, F) {
    let t634 = t626 * t633;
    let t637 = F::cast_from(0.65854491829355115987e0_f64) * t622 * t188 - F::cast_from(0.65854491829355115987e0_f64) * t184 * t634;
    (t634, t637)
}
