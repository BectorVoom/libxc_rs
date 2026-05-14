//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 959/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk959<F: Float>(t1137: F, t7648: F, t2105: F, t2029: F, t2916: F, t2923: F, t302: F, t2009: F, t287: F) -> (F, F, F, F, F, F) {
    let t7649 = t1137 * t7648;
    let t7650 = t2105 * t7649;
    let t7653 = t2916 * t2029;
    let t7654 = t7653 * t2923;
    let t7655 = t302 * t7654;
    let t7658 = t2009 * t287;
    (t7649, t7650, t7653, t7654, t7655, t7658)
}
