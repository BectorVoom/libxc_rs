//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2207/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2207(t4158: f64, t7950: f64, t18190: f64, t2042: f64, t1459: f64, t28271: f64, t5795: f64, t7334: f64, t1518: f64, t572: f64, t95137: f64, t26123: f64, t4292: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t101632 = 6.0_f64 * t4158 * t7950;
    let t101634 = 3.0_f64 * t18190 * t2042;
    let t101640 = 12.0_f64 * t1459 * t28271;
    let t101642 = 6.0_f64 * t5795 * t7334;
    let t101645 = 6.0_f64 * t572 * t95137 * t1518;
    let t101648 = 12.0_f64 * t572 * t26123 * t4292;
    (t101632, t101634, t101640, t101642, t101645, t101648)
}
