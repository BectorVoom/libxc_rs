//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1160/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1160(t12957: f64, t520: f64, t1224: f64, t774: f64, t10141: f64, t1222: f64, t12858: f64, t12861: f64, t12865: f64, t12869: f64, t12873: f64, t12877: f64, t12881: f64, t12883: f64, t12889: f64, t12891: f64, t12894: f64, t12898: f64, t12902: f64, t3271: f64, t4413: f64) -> (f64, f64, f64) {
    let t12958 = t12957 * t520;
    let t12960 = t1224 * t774 * t12958;
    let t12963 = -t10141 * t12858 / 4.0_f64 - 35.0_f64 / 216.0_f64 * t12861 + t3271 * t12865 / 384.0_f64 + t4413 * t12869 / 768.0_f64 + t3271 * t12873 / 768.0_f64 - t3271 * t12877 / 3072.0_f64 - t12881 - 5.0_f64 / 768.0_f64 * t3271 * t12883 - t12889 - t12891 * t12894 / 512.0_f64 + t4413 * t12898 / 512.0_f64 + t12902 - t1222 * t12960 / 3072.0_f64;
    (t12958, t12960, t12963)
}
