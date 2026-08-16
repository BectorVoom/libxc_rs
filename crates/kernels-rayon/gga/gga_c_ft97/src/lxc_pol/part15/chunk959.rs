//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 959/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk959(t1882: f64, t21686: f64, t21515: f64, t21510: f64, t8392: f64, t21541: f64, t21549: f64, t21682: f64, t1095: f64, t13411: f64, t17818: f64, t17836: f64, t17868: f64, t6757: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t79047 = t1882 * t21686;
    let t79138 = t1882 * t21515;
    let t79157 = t8392 * t21510;
    let t79179 = t1882 * t21541;
    let t79182 = t1882 * t21549;
    let t79218 = t1882 * t21682;
    let t79252 = t13411 * t1095;
    let t79253 = t79252 * t17818;
    let t79305 = t17836 * t17868 * t6757;
    (t79047, t79138, t79157, t79179, t79182, t79218, t79253, t79305)
}
