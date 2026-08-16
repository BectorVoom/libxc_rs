//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1213/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1213<F: Float>(t14202: F, t4414: F, t14340: F, t9270: F, t14286: F, t840: F, t1205: F, t19631: F, t829: F, t830: F, t4083: F, t4424: F) -> (F, F, F, F, F) {
    let t52309 = t4414 * t14202;
    let t52331 = t9270 * t14340;
    let t52345 = t840 * t14286;
    let t52348 = t19631 * t1205;
    let t52350 = t829 * t830 * t52348;
    let t52353 = t4424 * t4083;
    (t52309, t52331, t52345, t52350, t52353)
}
