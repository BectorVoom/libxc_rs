//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1245/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1245<F: Float>(t13792: F, t53245: F, t14617: F, t50936: F, t3316: F, t859: F, t1192: F, t20173: F, t14125: F, t3111: F, t833: F, t850: F) -> (F, F, F, F, F) {
    let t53246 = t13792 * t53245;
    let t53248 = t50936 * t14617;
    let t53250 = t859 * t3316;
    let t53251 = t13792 * t53250;
    let t53253 = t20173 * t1192;
    let t53260 = t850 * t3111 * t14125 * t833;
    (t53246, t53248, t53251, t53253, t53260)
}
