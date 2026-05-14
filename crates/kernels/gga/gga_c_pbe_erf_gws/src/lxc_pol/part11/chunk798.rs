//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 798/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk798<F: Float>(t13086: F, t376: F, t338: F, t353: F, t1144: F, t3896: F, t1105: F, t3721: F, t2409: F, t3067: F, t3737: F, t13290: F, t829: F, t830: F, t831: F, t13233: F, t13235: F, t13237: F, t13238: F, t13240: F, t13245: F, t13247: F, t13284: F, t13295: F, t13302: F, t13306: F, t13308: F, t13313: F) -> (F, F, F, F, F, F, F, F) {
    let t13639 = t376 * t13086;
    let t13641 = t338 * t353 * t13639;
    let t13645 = t338 * t1144 * t3896;
    let t13648 = t3721 * t1105;
    let t13650 = t2409 * t3067 * t13648;
    let t13656 = t338 * t1144 * t3737;
    let t13662 = t829 * t830 * t831 * t13290;
    let t13671 = -t13233 - t13235 - t13237 + t13238 - t13240 + t13245 + t13247 + t13284 + t13295 + t13302 - t13306 - t13308 - t13313;
    (t13639, t13641, t13645, t13648, t13650, t13656, t13662, t13671)
}
