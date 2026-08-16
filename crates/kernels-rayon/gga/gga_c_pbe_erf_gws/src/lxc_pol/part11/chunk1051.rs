//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1051/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1051(t1105: f64, t3855: f64, t2407: f64, t858: f64, t1134: f64, t3717: f64, t3825: f64, t3128: f64, t36837: f64, t13357: f64, t6203: f64, t13296: f64, t20944: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45126 = t3855 * t1105;
    let t45128 = t2407 * t858 * t45126;
    let t45133 = t2407 * t858 * t1134 * t3717;
    let t45140 = t2407 * t858 * t3825 * t1105;
    let t45190 = t3128 * t36837;
    let t45192 = t6203 * t13357;
    let t45194 = t20944 * t13296;
    (t45126, t45128, t45133, t45140, t45190, t45192, t45194)
}
