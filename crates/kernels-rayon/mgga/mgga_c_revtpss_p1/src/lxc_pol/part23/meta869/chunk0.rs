//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2766/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2766(t1412: f64, t6861: f64, t2661: f64, t3938: f64, t3992: f64, t5608: f64, t5659: f64, t1399: f64, t22025: f64, t22212: f64, t2496: f64, t1317: f64, t22193: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74026 = t1412 * t6861;
    let t74029 = t2661 * t3992 * t74026 * t3938;
    let t74033 = t2661 * t3992 * t5608 * t5659;
    let t74037 = t2661 * t3992 * t22025 * t1399;
    let t74106 = t22212 * t2496;
    let t74111 = t1317 * t22193;
    (t74026, t74029, t74033, t74037, t74106, t74111)
}
