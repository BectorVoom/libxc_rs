//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 329/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk329(t183: f64, t848: f64, t1004: f64, t453: f64, t377: f64, t457: f64, t310: f64, t460: f64, t452: f64, t864: f64, t1035: f64, t180: f64, t322: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1226 = 0.65854491829355115987e0_f64 * t848 * t183;
    let t1228 = 0.13170898365871023197e1_f64 * t1004 * t453;
    let t1229 = t377 * t457;
    let t1231 = t310 * t460;
    let t1233 = t452 * t864;
    let t1235 = 0.13170898365871023197e1_f64 * t1035 * t1233;
    let t1236 = t180 * t322;
    (t1226, t1228, t1229, t1231, t1233, t1235, t1236)
}
