//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 967/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk967<F: Float>(t13849: F, t2409: F, t9296: F, t13772: F, t13778: F, t13785: F, t13789: F, t13794: F, t13801: F, t13804: F, t13810: F, t13813: F, t13818: F, t13822: F, t13826: F, t13833: F, t13837: F, t13842: F, t13846: F, t2408: F, t3066: F, t3207: F, t335: F, t827: F) -> (F, F) {
    let t13851 = t2409 * t9296 * t13849;
    let t13854 = -t827 * t13772 / 48.0 - t13778 / 192.0 - t13785 / 768.0 - t13789 / 3072.0 - t13794 / 24.0 + t13801 / 1536.0 + t13804 / 1536.0 - t13810 + t13813 / 96.0 + t13818 / 96.0 - t3207 * t13822 / 16.0 - t335 * t13826 / 48.0 + 5.0 / 768.0 * t13833 + t2408 * t13837 / 24.0 + t3066 * t13842 / 48.0 + t2408 * t13846 / 24.0 - t3066 * t13851 / 16.0;
    (t13851, t13854)
}
