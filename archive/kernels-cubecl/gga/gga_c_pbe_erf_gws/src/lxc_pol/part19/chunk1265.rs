//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1265/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1265<F: Float>(t1161: F, t353: F, t52191: F, t859: F, t53952: F, t27729: F, t4082: F, t20154: F, t3067: F, t4207: F, t938: F, t53970: F) -> (F, F, F, F, F) {
    let t55722 = t859 * t353 * t52191 * t1161;
    let t55726 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t53952;
    let t55729 = t27729 * t4082;
    let t55734 = t20154 * t3067 * t4207 * t938;
    let t55739 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t53970;
    (t55722, t55726, t55729, t55734, t55739)
}
