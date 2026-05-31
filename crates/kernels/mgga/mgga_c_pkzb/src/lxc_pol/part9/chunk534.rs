//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 534/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk534<F: Float>(t2209: F, t834: F, t2172: F, t336: F) -> (F, F, F) {
    let t2210 = t834 * t2209;
    let t2212 = F::cast_from(0.39862222222222222223e0_f64) * t2172;
    let t2215 = F::cast_from(1.0_f64)/F::sqrt(t336);
    (t2210, t2212, t2215)
}
