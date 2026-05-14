//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1166/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1166<F: Float>(t29502: F, t4248: F, t2014: F, t22483: F, t7934: F, t1497: F, t29547: F, t77: F, t1493: F, t5816: F, t22656: F, t84: F, t101252: F, t101333: F, t101342: F, t108880: F, t108966: F, t108971: F, t108979: F, t108987: F, t108990: F, t25157: F, t28151: F, t28154: F, t29562: F, t92690: F) -> (F, F, F) {
    let t114230 = 12.0 * t4248 * t29502;
    let t114238 = 3.0 * t2014 * t7934 * t22483;
    let t114246 = t77 * t29547 * t1497;
    let t114260 = t77 * t1493 * t5816;
    let t114264 = t77 * t84 * t22656;
    let t114267 = 30.0 * t101252 * t108880 - 15.0 * t101333 * t29562 - 15.0 * t101342 * t29562 - 10.0 * t108966 * t28151 - 10.0 * t108971 * t28154 - 10.0 * t108979 * t28154 - 5.0 * t108987 * t28154 - 5.0 * t108990 * t28151 - 15.0 * t114246 * t25157 - 15.0 * t114260 * t25157 + 35.0 * t114264 * t92690;
    (t114230, t114238, t114267)
}
