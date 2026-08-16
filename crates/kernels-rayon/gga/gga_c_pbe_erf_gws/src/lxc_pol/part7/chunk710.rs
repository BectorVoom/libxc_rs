//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 710/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk710(t5805: f64, t5813: f64, t1513: f64, t5809: f64, t1544: f64, t156: f64, t496: f64, t506: f64, t5683: f64, t102: f64, t505: f64, t96: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5814 = t5813 * t5805;
    let t5815 = 0.2923025e1_f64 * t5814;
    let t5816 = t1513 * t5809;
    let t5817 = 0.19486833333333333333e1_f64 * t5816;
    let t5818 = t156 * t1544;
    let t5819 = t496 * t5818;
    let t5821 = t506 * t5683;
    let t5823 = 0.1753815e2_f64 * t102 * t5821;
    let t5825 = 1.0_f64 / t505 / t96;
    (t5815, t5817, t5818, t5819, t5821, t5823, t5825)
}
