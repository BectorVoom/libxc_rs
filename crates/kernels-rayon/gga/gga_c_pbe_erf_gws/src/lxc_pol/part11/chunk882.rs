//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 882/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk882(t1378: f64, t147: f64, t2331: f64, t6056: f64, t1952: f64, t4579: f64, t553: f64, t1971: f64, t4585: f64, t5697: f64, t6055: f64, t1368: f64, t19: f64) -> (f64, f64, f64, f64, f64) {
    let t16422 = 0.67015213385620818113e-4_f64 * t2331 * t147 * t1378 * t6056;
    let t16441 = 0.39507780657818961764e-1_f64 * t1952 * t4579 * t553;
    let t16444 = 0.13871971944573393855e-1_f64 * t5697 * t4585 * t1971;
    let t16446 = 0.2267957317922316773e-1_f64 * t6055 * t1971;
    let t16451 = t1368 * t19;
    (t16422, t16441, t16444, t16446, t16451)
}
