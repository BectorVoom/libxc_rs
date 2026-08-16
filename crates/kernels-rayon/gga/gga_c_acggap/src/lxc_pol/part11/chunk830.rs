//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 830/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk830(t1614: f64, t1960: f64, t119: f64, t2331: f64, t157: f64, t2122: f64, t524: f64, t2152: f64, t2333: f64, t310: f64, t1620: f64, t2127: f64, t2143: f64, t2146: f64, t2155: f64, t2351: f64, t464: f64, t7900: f64, t7901: f64, t7909: f64, t7921: f64, t7926: f64, t8995: f64, t8999: f64, t9003: f64) -> (f64, f64, f64) {
    let t9008 = t1960 * t1614;
    let t9010 = t119 * t2331;
    let t9014 = t2122 * t524 * t157;
    let t9015 = t2152 * t9014;
    let t9018 = t310 * t2333;
    let t9022 = 0.65854491829355115987e0_f64 * t119 * t8995 + 0.8673628188205199462e0_f64 * t8999 + t7900 + 0.65854491829355115987e0_f64 * t7901 - 0.17347256376410398924e1_f64 * t7909 + 0.4336814094102599731e0_f64 * t9003 * t2155 + 0.13170898365871023197e1_f64 * t2127 * t1620 + 0.65854491829355115987e0_f64 * t9008 - 0.65854491829355115987e0_f64 * t9010 * t464 + 0.4336814094102599731e0_f64 * t2146 * t9015 - t7921 + 0.65854491829355115987e0_f64 * t9018 - t7926 - 0.4336814094102599731e0_f64 * t2143 * t2351;
    (t9010, t9015, t9022)
}
