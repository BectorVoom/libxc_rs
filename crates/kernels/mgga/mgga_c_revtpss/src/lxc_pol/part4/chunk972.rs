//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 972/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk972<F: Float>(t11922: F, t3119: F, t3115: F, t1086: F, t3057: F, t3090: F, t11671: F, t3114: F, t127: F, t3206: F, t371: F, t3205: F, t11200: F, t225: F, t3218: F, t1025: F) -> (F, F, F, F, F, F) {
    let t11923 = t11922 * t3119;
    let t11924 = t3115 * t11923;
    let t11926 = t3057 * t1086;
    let t11927 = t11926 * t3090;
    let t11933 = t3114 * t11671;
    let t11937 = t371 * t127 * t3206;
    let t11938 = t3205 * t11937;
    let t11940 = t11200 * t225;
    let t11951 = t371 * t127 * t3218;
    let t11952 = t1025 * t11951;
    (t11924, t11927, t11933, t11938, t11940, t11952)
}
