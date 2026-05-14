//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1069/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1069<F: Float>(t1205: F, t19631: F, t829: F, t830: F, t4083: F, t4424: F, t14881: F, t2417: F, t353: F, t859: F, t4111: F, t4386: F, t810: F, t14186: F, t892: F, t14188: F, t19906: F) -> (F, F, F, F, F, F) {
    let t52348 = t19631 * t1205;
    let t52350 = t829 * t830 * t52348;
    let t52353 = t4424 * t4083;
    let t52381 = t859 * t353 * t14881 * t2417;
    let t52393 = t4386 * t353 * t4111 * t810;
    let t52417 = t859 * t892 * t14186;
    let t52432 = t19906 * t14188;
    (t52350, t52353, t52381, t52393, t52417, t52432)
}
