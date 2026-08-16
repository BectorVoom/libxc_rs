//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 303/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk303(t837: f64, t970: f64, t242: f64, t336: f64, t363: f64, t917: f64, t923: f64, t925: f64, t931: f64, t946: f64, t951: f64, t958: f64, t964: f64, t967: f64) -> (f64, f64) {
    let t971 = t970 * t837;
    let t972 = t242 * t971;
    let t975 = -t917 * t336 / 36.0_f64 + t923 + t925 * t931 / 288.0_f64 + t946 * t951 / 3072.0_f64 - t958 * t363 / 576.0_f64 + t964 + t967 * t972 / 4608.0_f64;
    (t972, t975)
}
