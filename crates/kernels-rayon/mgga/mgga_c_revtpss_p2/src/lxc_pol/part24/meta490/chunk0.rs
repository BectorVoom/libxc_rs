//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1485/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1485(t1412: f64, t6861: f64, t22212: f64, t2496: f64, t2626: f64, t1320: f64, t22195: f64, t22129: f64, t2713: f64, t3964: f64, t6856: f64, t9779: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74026 = t1412 * t6861;
    let t74106 = t22212 * t2496;
    let t74130 = t22212 * t2626;
    let t74132 = t1320 * t22195;
    let t74264 = t3964 * t2713 * t22129;
    let t74277 = t9779 * t6856;
    (t74026, t74106, t74130, t74132, t74264, t74277)
}
