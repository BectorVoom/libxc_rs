//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1994/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1994(t14304: f64, t4147: f64, t1868: f64, t4135: f64, t116: f64, t13424: f64, t10871: f64, t1558: f64, t2722: f64, t14772: f64, t221: f64, t2645: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49564 = t14304 * t4147;
    let t49582 = t1868 * t4135;
    let t49686 = t13424 * t116;
    let t50474 = t1558 * t10871;
    let t50511 = t1558 * t2722;
    let t50538 = t221 * t14772;
    let t50560 = t1558 * t2645;
    (t49564, t49582, t49686, t50474, t50511, t50538, t50560)
}
