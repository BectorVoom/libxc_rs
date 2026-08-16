//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 584/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk584(t9845: f64, t969: f64, t9829: f64, t825: f64, t2465: f64, t2571: f64, t2464: f64, t313: f64, t9725: f64, t9739: f64, t1645: f64, t9740: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9846 = 0.38342925953920749676e0_f64 * t9845;
    let t9847 = t969 * t9829;
    let t9848 = t825 * t9847;
    let t9849 = 0.38342925953920749676e0_f64 * t9848;
    let t9850 = t2465 * t2571;
    let t9851 = t2464 * t9850;
    let t9852 = t825 * t9851;
    let t9853 = 0.85206502119823888169e-1_f64 * t9852;
    let t9854 = t313 * t9725;
    let t9857 = t313 * t9739;
    let t9858 = t1645 * t9740;
    (t9846, t9848, t9849, t9852, t9853, t9854, t9857, t9858)
}
