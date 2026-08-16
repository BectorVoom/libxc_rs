//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1205/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1205(t19127: f64, t935: f64, t915: f64, t11294: f64, t6145: f64, t11465: f64, t6189: f64, t4733: f64, t981: f64, t11108: f64, t6400: f64, t1100: f64, t18902: f64, t19025: f64, t19027: f64, t19029: f64, t19031: f64, t19048: f64, t19051: f64, t19053: f64, t19055: f64, t19058: f64, t19060: f64, t19062: f64, t19079: f64, t19081: f64, t19084: f64, t5023: f64) -> (f64, f64, f64, f64) {
    let t19128 = t19127 * t935;
    let t19130 = 1.0_f64 * t915 * t19128;
    let t19132 = 0.16081979498692535067e2_f64 * t11294 * t6145;
    let t19133 = t11465 * t6189;
    let t19134 = t19133 * t4733;
    let t19136 = 0.10389515463408878255e3_f64 * t981 * t19134;
    let t19137 = t6400 * t11108;
    let t19141 = 2.0_f64 * t1100 * t19137 * t5023 - t18902 - t19025 - t19027 - t19029 + t19031 + t19048 - t19051 - t19053 + t19055 + t19058 + t19060 + t19062 - t19079 - t19081 - t19084 + t19130 + t19132 + t19136;
    (t19130, t19132, t19136, t19141)
}
