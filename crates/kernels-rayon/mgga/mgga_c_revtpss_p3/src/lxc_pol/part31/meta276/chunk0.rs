//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1238/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1238(t7301: f64, t7925: f64, t545: f64, t7910: f64, t2028: f64, t1904: f64, t2027: f64, t2030: f64, t213: f64, t561: f64, t7245: f64, t7248: f64, t7279: f64, t7288: f64, t7291: f64, t7295: f64, t7911: f64, t7917: f64, t7921: f64) -> (f64, f64, f64, f64) {
    let t7926 = t7301 * t7925;
    let t7929 = t545 * t7910;
    let t7930 = t2028 * t7929;
    let t7933 = -t7245 + t7248 + 0.65854491829355115987e0_f64 * t213 * t7911 * t561 - 0.65854491829355115987e0_f64 * t7279 * t1904 + t7288 - t7291 - 0.4336814094102599731e0_f64 * t7917 * t2030 + 0.8673628188205199462e0_f64 * t7295 * t7921 + 0.4336814094102599731e0_f64 * t7295 * t7926 - 0.4336814094102599731e0_f64 * t2027 * t7930;
    (t7926, t7929, t7930, t7933)
}
