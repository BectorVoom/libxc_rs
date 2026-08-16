//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 878/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk878(t16662: f64, t16665: f64, t16668: f64, t16756: f64, t16758: f64, t16760: f64, t16765: f64, t16768: f64, t16771: f64, t16775: f64, t16777: f64, t5322: f64, t633: f64) -> (f64, f64) {
    let t16778 = t16662 + t16665 - t16668 + t16756 - t16758 + t16760 + t16765 - t16768 - t16771 - t16775 - t16777;
    let t16780 = t633 * t5322;
    (t16778, t16780)
}
