//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 604/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk604<F: Float>(t2784: F, t598: F, t186: F, t185: F, t1004: F, t172: F, t184: F, t564: F, t1006: F, t612: F, t1883: F, t582: F, t996: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2785 = t598 * t2784;
    let t2786 = t186 * t2785;
    let t2788 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t185 * t2786;
    let t2789 = t172 * t1004;
    let t2790 = t2789 * t184;
    let t2792 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2790 * t564;
    let t2794 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1006 * t612;
    let t2795 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1883;
    let t2796 = t582 * t996;
    (t2785, t2786, t2788, t2789, t2790, t2792, t2794, t2795, t2796)
}
