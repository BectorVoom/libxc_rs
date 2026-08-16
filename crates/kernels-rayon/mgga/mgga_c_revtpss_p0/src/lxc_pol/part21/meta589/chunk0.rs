//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2306/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2306(t19979: f64, t372: f64, t1651: f64, t2857: f64, t2852: f64, t1774: f64, t3362: f64, t1794: f64, t3617: f64, t17394: f64, t4890: f64, t3767: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19980 = t372 * t19979;
    let t20094 = t1651 * t2857;
    let t20099 = t1651 * t2852;
    let t20921 = t1774 * t3362;
    let t20944 = t3617 * t1794;
    let t20945 = t372 * t20944;
    let t21013 = t17394 * t4890;
    let t21014 = t3767 * t21013;
    (t19980, t20094, t20099, t20921, t20945, t21013, t21014)
}
