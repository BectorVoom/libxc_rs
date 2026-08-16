//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 534/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk534(t1480: f64, t1483: f64, t2290: f64, t44: f64, t56: f64, t5835: f64, t5838: f64, t5843: f64, t5848: f64, t5851: f64, t61: f64, t38: f64) -> (f64, f64) {
    let t5854 = 5.0_f64 / 18.0_f64 * t44 * t5835 + 5.0_f64 / 6.0_f64 * t44 * t5838 + 88.0_f64 / 9.0_f64 * t5843 * t61 + 40.0_f64 / 9.0_f64 * t1480 * t1483 + 5.0_f64 / 18.0_f64 * t56 * t5848 - 5.0_f64 / 6.0_f64 * t56 * t5851 - t2290;
    let t5855 = t38 * t5854;
    (t5854, t5855)
}
