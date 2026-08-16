//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1225/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1225<F: Float>(t14138: F, t52990: F, t3093: F, t4386: F, t3089: F, t13972: F, t14443: F, t1123: F, t52033: F, t833: F, t850: F, t13808: F, t14698: F) -> (F, F, F, F, F, F) {
    let t52991 = t52990 * t14138;
    let t52993 = t4386 * t3093;
    let t52996 = t4386 * t3089;
    let t53011 = t13972 * t14443;
    let t53015 = t850 * t1123 * t52033 * t833;
    let t53060 = t13808 * t14698;
    (t52991, t52993, t52996, t53011, t53015, t53060)
}
