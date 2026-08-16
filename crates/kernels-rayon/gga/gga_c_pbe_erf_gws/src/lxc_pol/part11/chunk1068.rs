//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1068/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1068(t13229: f64, t4414: f64, t12198: f64, t2503: f64, t13677: f64, t376: f64, t829: f64, t830: f64, t1114: f64, t13140: f64, t2365: f64, t833: f64) -> (f64, f64, f64, f64) {
    let t46731 = t4414 * t13229;
    let t46759 = t12198 * t2503;
    let t46763 = t829 * t830 * t13677 * t376;
    let t46858 = t1114 * t13140 * t2365 * t833;
    (t46731, t46759, t46763, t46858)
}
