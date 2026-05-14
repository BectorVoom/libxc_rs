//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 869/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk869<F: Float>(t1518: F, t26123: F, t572: F, t4292: F, t7330: F, t1459: F, t7953: F, t116: F, t7741: F, t670: F, t117: F, t28042: F, t1461: F, t1918: F, t2040: F, t28246: F, t28257: F, t28259: F, t28261: F, t28263: F, t28267: F, t573: F, t5802: F, t5805: F, t7324: F, t7944: F) -> (F, F, F, F, F) {
    let t28268 = t26123 * t1518;
    let t28270 = 6.0 * t572 * t28268;
    let t28271 = t7330 * t4292;
    let t28273 = 6.0 * t572 * t28271;
    let t28275 = 3.0 * t1459 * t7953;
    let t28276 = t116 * t7741;
    let t28277 = t28276 * t670;
    let t28279 = 6.0 * t572 * t28277;
    let t28280 = t117 * t28042;
    let t28282 = 3.0 * t572 * t28280;
    let t28283 = 3.0 * t1461 * t7944 + 3.0 * t1918 * t7324 + 6.0 * t2040 * t5802 + 3.0 * t2040 * t5805 + t28246 * t573 + t28257 + t28259 + t28261 + t28263 + t28267 + t28270 + t28273 + t28275 + t28279 + t28282;
    (t28268, t28271, t28277, t28280, t28283)
}
