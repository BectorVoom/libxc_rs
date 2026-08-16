//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1931/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1931(t103421: f64, t7058: f64, t11064: f64, t8019: f64, t28993: f64, t571: f64, t2118: f64, t5789: f64, t1464: f64, t8113: f64, t1913: f64, t7560: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t103547 = t7058 * t103421;
    let t103586 = t8019 * t11064;
    let t104062 = 2.0_f64 * t571 * t28993;
    let t104071 = 2.0_f64 * t5789 * t2118;
    let t104073 = 2.0_f64 * t8113 * t1464;
    let t104077 = 2.0_f64 * t1913 * t7560;
    (t103547, t103586, t104062, t104071, t104073, t104077)
}
