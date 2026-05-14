//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 447/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk447<F: Float>(t1802: F, t188: F, t610: F, t186: F, t185: F, t219: F, t642: F) -> (F, F, F, F, F) {
    let t1803 = t188 * t1802;
    let t1804 = t610 * t610;
    let t1805 = t1803 * t1804;
    let t1806 = t186 * t1805;
    let t1808 = 4.0 / 15.0 * t185 * t1806;
    let t1809 = t642 * t219;
    (t1804, t1805, t1806, t1808, t1809)
}
