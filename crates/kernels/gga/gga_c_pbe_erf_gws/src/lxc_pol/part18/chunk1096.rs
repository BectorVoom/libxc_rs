//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1096/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1096<F: Float>(t1176: F, t21518: F, t367: F, t14602: F, t51666: F, t14460: F, t4414: F, t2416: F, t4182: F, t353: F, t859: F, t938: F, t14415: F, t51563: F, t14397: F, t2367: F) -> (F, F, F, F, F, F, F) {
    let t53592 = t1176 * t367 * t21518;
    let t53597 = t51666 * t14602;
    let t53598 = 7.0 / 576.0 * t53597;
    let t53610 = 7.0 / 72.0 * t4414 * t14460;
    let t53614 = t2416 * t4182;
    let t53617 = t859 * t353 * t53614 * t938;
    let t53625 = t51563 * t14415;
    let t53626 = 7.0 / 1152.0 * t53625;
    let t53629 = 7.0 / 144.0 * t2367 * t14397;
    (t53592, t53598, t53610, t53614, t53617, t53626, t53629)
}
