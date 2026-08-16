//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1142/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1142(t532: f64, t8107: f64, t1450: f64, t2107: f64, t5542: f64, t118: f64, t1502: f64, t1519: f64, t1843: f64, t1911: f64, t2014: f64, t2052: f64, t2056: f64, t2089: f64, t2093: f64, t2108: f64, t4248: f64, t508: f64, t569: f64, t651: f64, t7359: f64, t7732: f64, t7898: f64, t7969: f64, t7978: f64, t7984: f64, t7988: f64, t8065: f64, t8075: f64, t8079: f64) -> (f64, f64, f64, f64) {
    let t8108 = t532 * t8107;
    let t8109 = t8108 * t1450;
    let t8111 = t2107 * t5542;
    let t8113 = -t118 * t8065 - t1502 * t2089 - 2.0_f64 * t1519 * t7359 - t1843 * t2052 + t1911 * t2093 + 3.0_f64 * t2014 * t8079 + t2014 * t8109 - t2014 * t8111 - 2.0_f64 * t2056 * t4248 - 2.0_f64 * t2056 * t7732 + t2108 * t7898 - t508 * t7969 + t569 * t8075 - 2.0_f64 * t651 * t7978 - 2.0_f64 * t651 * t7984 - 2.0_f64 * t651 * t7988;
    (t8108, t8109, t8111, t8113)
}
