//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1231/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1231<F: Float>(t52991: F, t3093: F, t4386: F, t3089: F, t13972: F, t14443: F, t1123: F, t52033: F, t833: F, t850: F, t14711: F, t8801: F) -> (F, F, F, F, F, F) {
    let t52992 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t52991;
    let t52993 = t4386 * t3093;
    let t52996 = t4386 * t3089;
    let t53011 = t13972 * t14443;
    let t53012 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t53011;
    let t53015 = t850 * t1123 * t52033 * t833;
    let t53025 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t8801 * t14711;
    (t52992, t52993, t52996, t53012, t53015, t53025)
}
