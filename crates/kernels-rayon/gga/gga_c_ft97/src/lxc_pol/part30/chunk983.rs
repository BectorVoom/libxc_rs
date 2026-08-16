//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 983/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk983(t1882: f64, t34062: f64, t34067: f64, t34227: f64, t34139: f64, t312: f64, t33953: f64, t34169: f64, t34174: f64, t34337: f64, t5: f64, t140582: f64, t6749: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t144246 = t1882 * t34062;
    let t144248 = t1882 * t34067;
    let t144250 = t1882 * t34227;
    let t144260 = t1882 * t34139;
    let t144262 = t312 * t33953;
    let t144271 = t1882 * t34169;
    let t144273 = t1882 * t34174;
    let t144289 = t5 * t34337;
    let t149674 = t140582 * t6749;
    (t144246, t144248, t144250, t144260, t144262, t144271, t144273, t144289, t149674)
}
