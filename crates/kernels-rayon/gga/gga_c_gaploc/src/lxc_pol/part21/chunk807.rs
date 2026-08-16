//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 807/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk807(t1890: f64, t2530: f64, t590: f64, t1457: f64, t7250: f64, t7254: f64, t1: f64, t106: f64, t316: f64, t2154: f64, t774: f64, t959: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7696 = t1890 * t2530;
    let t7697 = t7696 * t590;
    let t7700 = t1457 * t7250;
    let t7703 = t1457 * t7254;
    let t7710 = t2530 * t1;
    let t7711 = t7710 * t106;
    let t7712 = t7711 * t316;
    let t7715 = t2154 * t774;
    let t7716 = t7715 * t959;
    (t7696, t7697, t7700, t7703, t7712, t7716)
}
