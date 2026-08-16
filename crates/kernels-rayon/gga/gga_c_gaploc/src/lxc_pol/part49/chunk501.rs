//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 501/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk501(t1858: f64, t935: f64, t1890: f64, t7291: f64, t739: f64, t7068: f64, t1: f64, t2530: f64, t106: f64, t316: f64, t325: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7634 = t1858 * t935;
    let t7659 = t1890 * t7291;
    let t7667 = t739 * t7291;
    let t7671 = t739 * t7068;
    let t7710 = t2530 * t1;
    let t7711 = t7710 * t106;
    let t7712 = t7711 * t316;
    let t7784 = t883 * t325;
    (t7634, t7659, t7667, t7671, t7712, t7784)
}
