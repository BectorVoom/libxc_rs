//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1093/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1093<F: Float>(t13792: F, t53250: F, t1192: F, t20173: F, t14125: F, t3111: F, t833: F, t850: F, t1162: F, t13796: F, t2190: F, t3989: F, t3952: F, t8751: F, t14423: F, t14682: F, t2158: F) -> (F, F, F, F, F, F) {
    let t53251 = t13792 * t53250;
    let t53253 = t20173 * t1192;
    let t53260 = t850 * t3111 * t14125 * t833;
    let t53261 = 7.0 / 144.0 * t53260;
    let t53264 = t3989 * t13796 * t1162 * t2190;
    let t53266 = t3952 * t8751;
    let t53270 = t3989 * t14682 * t14423 * t2158;
    (t53251, t53253, t53261, t53264, t53266, t53270)
}
