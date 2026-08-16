//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 786/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk786<F: Float>(t512: F, t131: F, t120: F, t133: F, t1365: F, t5783: F, t5818: F, t5787: F, t1590: F, t524: F, t142: F, t1378: F, t1971: F, t5701: F) -> (F, F, F, F, F, F, F) {
    let t5852 = t512 * t512;
    let t5853 = F::cast_from(1.0_f64) / t5852;
    let t5854 = t131 * t5853;
    let t5863 = F::cast_from(0.89405814814814814813e0_f64) * t133 * t1365 * t120;
    let t5864 = t133 * t5783;
    let t5866 = t133 * t5818;
    let t5874 = t133 * t5787;
    let t5887 = t524 * t1590;
    let t5888 = t5887 * t142;
    let t5891 = t5701 * t1378 * t1971;
    (t5854, t5863, t5864, t5866, t5874, t5888, t5891)
}
