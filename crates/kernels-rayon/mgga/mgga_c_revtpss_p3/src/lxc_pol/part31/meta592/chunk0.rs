//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2017/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2017(t1399: f64, t2434: f64, t25880: f64, t25899: f64, t2022: f64, t9646: f64, t9648: f64, t25875: f64, t94394: f64, t46361: f64, t545: f64, t9685: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94633 = t2434 * t1399;
    let t94634 = t25880 * t94633;
    let t94635 = t25899 * t94634;
    let t94648 = 0.19637199382202157274e-3_f64 * t9646 * t2022 * t9648;
    let t94649 = t25875 * t94394;
    let t94656 = t46361 * t545;
    let t94661 = t25880 * t9685;
    (t94634, t94635, t94648, t94649, t94656, t94661)
}
