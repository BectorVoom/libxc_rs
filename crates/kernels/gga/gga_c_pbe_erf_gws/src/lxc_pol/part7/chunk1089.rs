//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1089/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1089<F: Float>(t2059: F, t2060: F, t279: F, t6045: F, t19: F, t6067: F, t796: F, t801: F, t116: F, t366: F, t798: F, t799: F) -> (F, F, F) {
    let t19517 = F::new(0.16521134411652656606e2) * t2059 * t2060 * t6045 * t279;
    let t19520 = t6067 * t796 * t19 * t801;
    let t19521 = F::new(0.16430531536026666667e1) * t19520;
    let t19525 = F::new(0.6693920255418271605e1) * t798 * t799 * t366 * t116;
    (t19517, t19521, t19525)
}
