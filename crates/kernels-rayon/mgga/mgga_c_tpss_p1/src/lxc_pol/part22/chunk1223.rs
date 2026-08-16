//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1223/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1223(t1692: f64, t17921: f64, t17931: f64, t17934: f64, t17938: f64, t18053: f64, t18056: f64, t18059: f64, t1812: f64, t18728: f64, t18803: f64, t18807: f64, t18812: f64, t1991: f64, t2439: f64, t30: f64, t3552: f64, t5539: f64, t5591: f64, t580: f64, t5849: f64, t5853: f64) -> f64 {
    let t18823 = 3.0_f64 * t3552 * t1812 * t17921 + 3.0_f64 * t2439 * t5849 * t5539 - 3.0_f64 * t18728 * t17931 + 3.0_f64 * t2439 * t1812 * t17934 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t17938 + t1692 * t18803 * t30 / 2.0_f64 - t1692 * t18807 * t5591 + t1692 * t5849 * t580 + t1692 * t18812 * t18053 - t1692 * t5853 * t18056 - t1692 * t5853 * t18059 / 2.0_f64 + t1692 * t1812 * t1991 / 2.0_f64;
    t18823
}
