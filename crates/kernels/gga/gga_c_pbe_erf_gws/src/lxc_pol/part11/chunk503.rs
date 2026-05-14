//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 503/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk503<F: Float>(t1803: F, t3454: F, t186: F, t185: F, t225: F, t3379: F) -> (F, F, F, F) {
    let t3455 = t1803 * t3454;
    let t3456 = t186 * t3455;
    let t3458 = 4.0 / 15.0 * t185 * t3456;
    let t3459 = t3379 * t225;
    (t3455, t3456, t3458, t3459)
}
