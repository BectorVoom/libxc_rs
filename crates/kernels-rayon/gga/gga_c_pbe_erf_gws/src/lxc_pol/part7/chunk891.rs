//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 891/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk891(t1698: f64, t1724: f64, t1815: f64, t639: f64, t5024: f64, t5522: f64, t661: f64, t1648: f64, t4924: f64, t1740: f64, t1775: f64, t5502: f64, t7011: f64) -> (f64, f64, f64, f64, f64) {
    let t16921 = 16.0_f64 / 15.0_f64 * t639 * t1815 * t1698 * t1724;
    let t16925 = 32.0_f64 / 9.0_f64 * t639 * t5522 * t5024 * t661;
    let t16927 = 32.0_f64 / 15.0_f64 * t1648 * t4924;
    let t16928 = t1775 * t1740;
    let t16929 = 16.0_f64 / 15.0_f64 * t16928;
    let t16931 = 16.0_f64 / 5.0_f64 * t7011 * t5502;
    (t16921, t16925, t16927, t16929, t16931)
}
