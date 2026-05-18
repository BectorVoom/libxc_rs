//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1238/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1238<F: Float>(t4146: F, t51818: F, t14592: F, t50994: F, t14749: F, t9270: F, t14643: F, t840: F, t14793: F, t1144: F, t13909: F, t859: F) -> (F, F, F, F, F, F) {
    let t53334 = t51818 * t4146;
    let t53353 = t50994 * t14592;
    let t53354 = F::new(7.0) / F::new(288.0) * t53353;
    let t53374 = F::new(7.0) / F::new(72.0) * t9270 * t14749;
    let t53405 = F::new(7.0) / F::new(144.0) * t840 * t14643;
    let t53407 = F::new(7.0) / F::new(24.0) * t9270 * t14793;
    let t53419 = t859 * t1144 * t13909;
    (t53334, t53354, t53374, t53405, t53407, t53419)
}
