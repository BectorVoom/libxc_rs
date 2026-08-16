//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1230/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1230<F: Float>(t13792: F, t52993: F, t3089: F, t4386: F, t1162: F, t14682: F, t2158: F, t3989: F, t13972: F, t14443: F, t1123: F, t52033: F, t833: F, t850: F) -> (F, F, F, F, F) {
    let t52994 = t13792 * t52993;
    let t52996 = t4386 * t3089;
    let t52997 = t13792 * t52996;
    let t53009 = t3989 * t14682 * t1162 * t2158;
    let t53011 = t13972 * t14443;
    let t53012 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t53011;
    let t53015 = t850 * t1123 * t52033 * t833;
    (t52994, t52997, t53009, t53012, t53015)
}
