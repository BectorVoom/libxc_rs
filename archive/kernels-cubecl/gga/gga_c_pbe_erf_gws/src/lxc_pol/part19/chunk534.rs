//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 534/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk534<F: Float>(t1809: F, t2673: F, t639: F, t1640: F, t219: F, t1642: F, t954: F, t422: F) -> (F, F, F, F, F) {
    let t2674 = t1809 * t2673;
    let t2676 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t639 * t2674;
    let t2677 = t1640 * t219;
    let t2678 = t1642 * t954;
    let t2679 = t2678 * t422;
    (t2674, t2676, t2677, t2678, t2679)
}
