//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1008/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1008<F: Float>(t5: F, t25167: F, t117: F, t4144: F, t9593: F, t2034: F, t2014: F, t10416: F, t1937: F, t13435: F, t2322: F, t6993: F, t196: F, t197: F, t3821: F, t2035: F, t531: F, t7311: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t25168 = piecewise3(t8, 0.0, t25167);
    let t25169 = t25168 * t117;
    let t25177 = t9593 * t4144;
    let t25178 = t2034 * t25177;
    let t25180 = 2.0 * t2014 * t25178;
    let t25182 = 2.0 * t10416 * t1937;
    let t25184 = 4.0 * t13435 * t1937;
    let t25186 = 4.0 * t2322 * t6993;
    let t25188 = t3821 * t196 * t197;
    let t25189 = t25188 * t2035;
    let t25190 = t531 * t7311;
    (t25168, t25169, t25177, t25178, t25180, t25182, t25184, t25186, t25188, t25189, t25190)
}
