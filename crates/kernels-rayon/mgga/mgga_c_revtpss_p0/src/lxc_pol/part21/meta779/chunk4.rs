//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2778/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2778(t10905: f64, t14825: f64, t14829: f64, t14819: f64, t40517: f64, t10811: f64, t14910: f64, t4423: f64, t836: f64, t14741: f64, t2710: f64, t2713: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51026 = t10905 * t14825;
    let t51028 = t10905 * t14829;
    let t51042 = t40517 * t14819;
    let t51047 = t10811 * t14910;
    let t51049 = t4423 * t836;
    let t51055 = t2710 * t2713 * t14741;
    (t51026, t51028, t51042, t51047, t51049, t51055)
}
