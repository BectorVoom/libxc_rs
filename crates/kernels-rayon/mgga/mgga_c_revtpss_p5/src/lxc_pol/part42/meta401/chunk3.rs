//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1367/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1367(t1248: f64, t13045: f64, t20956: f64, t3720: f64, t5341: f64, t1219: f64, t6667: f64, t247: f64, t3634: f64, t6429: f64, t1261: f64, t12856: f64, t20795: f64) -> (f64, f64, f64, f64, f64) {
    let t20957 = t13045 * t1248;
    let t20958 = t20956 * t20957;
    let t20959 = t3720 * t20958;
    let t20962 = t20956 * t5341;
    let t20963 = t3720 * t20962;
    let t20966 = t6667 * t1219;
    let t20973 = t247 * t3634 * t6429;
    let t20974 = t1261 * t20973;
    let t20977 = t20795 * t12856;
    (t20959, t20963, t20966, t20974, t20977)
}
