//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1209/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1209(t13899: f64, t3979: f64, t13996: f64, t9270: f64, t1176: f64, t2332: f64, t903: f64, t3993: f64, t20091: f64, t4009: f64, t13788: f64, t13972: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51807 = t3979 * t13899;
    let t51815 = t9270 * t13996;
    let t51818 = t1176 * t2332 * t903;
    let t51819 = t51818 * t3993;
    let t51825 = t20091 * t4009;
    let t51827 = t13972 * t13788;
    (t51807, t51815, t51818, t51819, t51825, t51827)
}
