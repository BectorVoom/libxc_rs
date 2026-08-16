//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 546/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk546(t480: f64, t5326: f64, t3623: f64, t4890: f64, t3782: f64, t1794: f64, t3153: f64, t3767: f64, t73: f64, t140: f64, t1781: f64, t1222: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5327 = t5326 * t480;
    let t5330 = t3623 * t4890;
    let t5331 = t3782 * t5330;
    let t5332 = t1794 * t3153;
    let t5340 = t3767 * t5330;
    let t5351 = t1794 * t73;
    let t5357 = t140 * t1781;
    let t5358 = t1222 * t5357;
    (t5327, t5330, t5331, t5332, t5340, t5351, t5357, t5358)
}
