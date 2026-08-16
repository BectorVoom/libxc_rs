//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1348/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1348<F: Float>(t15034: F, t859: F, t892: F, t1161: F, t353: F, t52191: F, t53952: F, t27729: F, t4082: F, t20154: F, t3067: F, t4207: F, t938: F) -> (F, F, F, F, F) {
    let t55717 = t859 * t892 * t15034;
    let t55722 = t859 * t353 * t52191 * t1161;
    let t55726 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t53952;
    let t55729 = t27729 * t4082;
    let t55734 = t20154 * t3067 * t4207 * t938;
    (t55717, t55722, t55726, t55729, t55734)
}
