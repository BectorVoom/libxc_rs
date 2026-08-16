//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 746/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk746<F: Float>(t789: F, t306: F, t5952: F, t2021: F, t271: F) -> (F, F, F, F) {
    let t5999 = t789 * t789;
    let t6000 = F::cast_from(1.0_f64) / t5999;
    let t6009 = t5952 * t306;
    let t6012 = F::cast_from(1.0_f64) / t2021 / t271;
    (t5999, t6000, t6009, t6012)
}
