//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 263/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk263<F: Float>(t312: F, t793: F, t309: F, t311: F, t19: F, t305: F, t20: F, t5: F) -> (F, F, F, F) {
    let t794 = t793 * t312;
    let t796 = 1.0 / t311 / t309;
    let t798 = t305 * t796 * t19;
    let t799 = t20 * t5;
    (t794, t796, t798, t799)
}
