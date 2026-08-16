//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 808/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk808(t6638: f64, t858: f64, t3065: f64, t6678: f64, t2263: f64, t358: f64, t356: f64, t2252: f64, t2157: f64, t6395: f64, t851: f64, t2255: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6679 = t858 * t6638;
    let t6680 = t3065 * t6679;
    let t6682 = t6678 * t6680 / 32.0_f64;
    let t6683 = t358 * t2263;
    let t6684 = t356 * t6683;
    let t6685 = t6684 * t2252;
    let t6686 = t6395 * t2157;
    let t6687 = t851 * t6686;
    let t6688 = t2255 * t6687;
    (t6679, t6680, t6682, t6684, t6685, t6686, t6688)
}
