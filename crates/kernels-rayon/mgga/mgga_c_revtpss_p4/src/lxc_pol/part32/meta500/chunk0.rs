//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1785/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1785(t28699: f64, t28729: f64, t28759: f64, t28942: f64, t3: f64, t2055: f64, t670: f64, t1518: f64, t26733: f64, t4292: f64, t7553: f64, t116: f64, t7983: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28944 = t28699 + t28729 + t28759 + t28942;
    let t28945 = t3 * t28944;
    let t28956 = param_d * t28944;
    let t28974 = t670 * t2055;
    let t28975 = t28974 * t1518;
    let t28978 = t26733 * t1518;
    let t28981 = t7553 * t4292;
    let t28986 = t116 * t7983;
    (t28944, t28945, t28956, t28974, t28975, t28978, t28981, t28986)
}
