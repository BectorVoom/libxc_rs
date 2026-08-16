//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1427/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1427(t17353: f64, t17514: f64, t1802: f64, t3147: f64, t3597: f64, t3594: f64, t1244: f64, t1214: f64, t4186: f64, t5296: f64, t1042: f64, t1469: f64, t3584: f64) -> (f64, f64, f64, f64, f64) {
    let t17515 = t17353 * t17514;
    let t17523 = t1802 * t3147;
    let t17524 = t3597 * t17523;
    let t17525 = t3594 * t17524;
    let t17528 = t1244 * t17523;
    let t17529 = t3594 * t17528;
    let t17534 = t4186 * t1214;
    let t17535 = t5296 * t17534;
    let t17536 = t1042 * t17535;
    let t17539 = t1469 * t3584;
    (t17515, t17525, t17529, t17536, t17539)
}
