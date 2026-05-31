//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1238/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1238<F: Float>(t4146: F, t51818: F, t14592: F, t50994: F, t14749: F, t9270: F, t14643: F, t840: F, t14793: F, t1144: F, t13909: F, t859: F) -> (F, F, F, F, F, F) {
    let t53334 = t51818 * t4146;
    let t53353 = t50994 * t14592;
    let t53354 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t53353;
    let t53374 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t9270 * t14749;
    let t53405 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t840 * t14643;
    let t53407 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t9270 * t14793;
    let t53419 = t859 * t1144 * t13909;
    (t53334, t53354, t53374, t53405, t53407, t53419)
}
