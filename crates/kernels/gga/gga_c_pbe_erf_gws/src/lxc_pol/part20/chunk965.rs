//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 965/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk965<F: Float>(t3820: F, t6484: F, t12056: F, t12057: F, t12060: F, t12061: F, t12067: F, t12071: F, t12078: F, t12082: F, t12086: F, t12088: F, t2277: F, t6718: F, t9669: F, t11449: F, t11489: F, t11538: F, t11595: F, t11638: F, t11676: F, t11742: F, t11790: F, t11839: F, t11871: F, t11899: F, t11941: F, t11973: F, t12003: F, t12053: F) -> (F, F) {
    let t12092 = t6484 * t3820;
    let t12093 = 7.0 / 72.0 * t12092;
    let t12094 = -t12056 + 7.0 / 2304.0 * t12057 + 119.0 / 3456.0 * t9669 + t12060 - 7.0 / 2304.0 * t12061 - t12067 - t12071 + t12078 + t12082 - t12086 - t2277 * t12088 / 1536.0 + 119.0 / 6912.0 * t6718 - t12093;
    let t12098 = t11449 + t11489 + t11538 + t11595 + t11638 + t11676 + t11742 + t11790 + t11839 + t11871 + t11899 + t11941 + t11973 + t12003 + t12053 + t12094;
    (t12093, t12098)
}
