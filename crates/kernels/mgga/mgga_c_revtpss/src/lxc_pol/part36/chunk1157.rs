//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1157/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1157<F: Float>(t30993: F, t571: F, t2167: F, t6951: F, t1913: F, t8249: F, t29508: F, t7742: F, t29502: F, t7732: F, t30123: F, t98450: F, t2014: F, t22475: F, t7934: F, t29996: F, t7898: F) -> (F, F, F, F, F, F, F, F) {
    let t113025 = t571 * t30993;
    let t113053 = t2167 * t6951;
    let t113054 = t1913 * t8249;
    let t113063 = 6.0 * t29508 * t7742;
    let t113065 = 12.0 * t7732 * t29502;
    let t113067 = 18.0 * t98450 * t30123;
    let t113076 = 6.0 * t2014 * t7934 * t22475;
    let t113078 = 6.0 * t7898 * t29996;
    (t113025, t113053, t113054, t113063, t113065, t113067, t113076, t113078)
}
