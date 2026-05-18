//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1234/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1234<F: Float>(t13792: F, t53245: F, t14617: F, t50936: F, t3316: F, t859: F, t14125: F, t3111: F, t833: F, t850: F, t1162: F, t13796: F, t2190: F, t3989: F) -> (F, F, F, F, F) {
    let t53246 = t13792 * t53245;
    let t53248 = t50936 * t14617;
    let t53250 = t859 * t3316;
    let t53251 = t13792 * t53250;
    let t53260 = t850 * t3111 * t14125 * t833;
    let t53264 = t3989 * t13796 * t1162 * t2190;
    (t53246, t53248, t53251, t53260, t53264)
}
