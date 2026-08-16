//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 317/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk317<F: Float>(t1054: F, t626: F, t1045: F, t184: F, t188: F) -> (F, F) {
    let t1055 = t626 * t1054;
    let t1058 = F::cast_from(0.65854491829355115987e0_f64) * t1045 * t188 - F::cast_from(0.65854491829355115987e0_f64) * t184 * t1055;
    (t1055, t1058)
}
