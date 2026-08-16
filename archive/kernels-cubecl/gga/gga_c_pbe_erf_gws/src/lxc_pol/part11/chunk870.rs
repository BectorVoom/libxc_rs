//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 870/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk870<F: Float>(t13086: F, t376: F, t338: F, t353: F, t1144: F, t3896: F, t1105: F, t3721: F, t2409: F, t3067: F, t3737: F, t13290: F, t829: F, t830: F, t831: F) -> (F, F, F, F, F, F, F) {
    let t13639 = t376 * t13086;
    let t13641 = t338 * t353 * t13639;
    let t13645 = t338 * t1144 * t3896;
    let t13648 = t3721 * t1105;
    let t13650 = t2409 * t3067 * t13648;
    let t13656 = t338 * t1144 * t3737;
    let t13662 = t829 * t830 * t831 * t13290;
    (t13639, t13641, t13645, t13648, t13650, t13656, t13662)
}
