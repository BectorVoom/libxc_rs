//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1232/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1232<F: Float>(t14602: F, t51666: F, t14415: F, t51563: F, t14127: F, t2503: F, t51530: F, t13791: F, t3039: F, t1144: F, t4387: F, t859: F) -> (F, F, F, F, F, F) {
    let t53597 = t51666 * t14602;
    let t53625 = t51563 * t14415;
    let t53645 = t14127 * t2503;
    let t53666 = F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t51530;
    let t53688 = t3039 * t13791;
    let t53699 = t859 * t1144 * t4387;
    (t53597, t53625, t53645, t53666, t53688, t53699)
}
