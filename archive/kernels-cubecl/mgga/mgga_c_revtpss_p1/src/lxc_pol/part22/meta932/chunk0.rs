//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3161/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3161<F: Float>(t13014: F, t5373: F, t12998: F, t1222: F, t140: F, t17404: F, t12941: F, t5293: F, t5274: F, t1263: F, t16750: F, t17547: F, t3704: F) -> (F, F, F, F, F, F, F) {
    let t57290 = t5373 * t13014;
    let t57292 = t5373 * t12998;
    let t57295 = t1222 * t140 * t17404;
    let t57297 = t5293 * t12941;
    let t57299 = t5274 * t12941;
    let t57303 = t1263 * t16750;
    let t57314 = t17547 * t3704;
    (t57290, t57292, t57295, t57297, t57299, t57303, t57314)
}
