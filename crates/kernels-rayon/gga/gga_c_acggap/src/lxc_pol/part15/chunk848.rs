//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 848/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk848(t157: f64, t2385: f64, t524: f64, t2152: f64, t159: f64, t619: f64, t9971: f64, t119: f64, t2146: f64, t2338: f64, t2400: f64, t2404: f64, t616: f64, t8067: f64, t8087: f64, t8096: f64, t8098: f64, t8106: f64, t8113: f64, t9003: f64, t9129: f64, t9155: f64, t9160: f64, t9169: f64, t9172: f64, t9973: f64, t9977: f64, t9982: f64, t9986: f64) -> (f64, f64, f64) {
    let t9990 = t2385 * t524 * t157;
    let t9991 = t2152 * t9990;
    let t9995 = t619 * t159 * t9971;
    let t10004 = t8067 - 0.8673628188205199462e0_f64 * t2338 * t2404 + 0.13170898365871023197e1_f64 * t9129 + 0.65854491829355115987e0_f64 * t119 * t9973 + t8087 - 0.26020884564615598386e1_f64 * t2146 * t9977 - 0.8673628188205199462e0_f64 * t2146 * t9982 + 0.17347256376410398924e1_f64 * t2146 * t9986 + 0.8673628188205199462e0_f64 * t2146 * t9991 - 0.4336814094102599731e0_f64 * t616 * t9995 - t8096 + 0.8673628188205199462e0_f64 * t9003 * t2400 - 0.34694512752820797848e1_f64 * t9155 - t8098 - t8106 + 0.34694512752820797848e1_f64 * t9160 + 0.17347256376410398924e1_f64 * t9169 - 0.17347256376410398924e1_f64 * t9172 - t8113;
    (t9991, t9995, t10004)
}
