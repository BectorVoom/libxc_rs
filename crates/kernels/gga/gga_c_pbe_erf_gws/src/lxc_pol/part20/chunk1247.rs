//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1247/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1247<F: Float>(t27047: F, t3067: F, t4164: F, t814: F, t9296: F, t938: F, t1112: F, t361: F, t51020: F, t3209: F, t51682: F, t3958: F, t6148: F) -> (F, F, F, F, F) {
    let t53790 = t27047 * t3067 * t4164 * t814;
    let t53795 = t27047 * t9296 * t4164 * t938;
    let t53799 = t361 * t51020 * t1112;
    let t53806 = t51682 * t3209;
    let t53807 = F::new(7.0) / F::new(24.0) * t53806;
    let t53840 = t3958 * t6148;
    (t53790, t53795, t53799, t53807, t53840)
}
