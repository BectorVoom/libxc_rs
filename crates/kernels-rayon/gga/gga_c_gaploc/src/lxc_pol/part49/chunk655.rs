//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 655/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk655(t10677: f64, t550: f64, t1843: f64, t10627: f64, t688: f64, t779: f64, t2508: f64, t296: f64, t3431: f64, t123: f64, t734: f64, t2554: f64, t2932: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10678 = t550 * t10677;
    let t10679 = t1843 * t10678;
    let t10682 = t10627 * t688;
    let t10683 = t779 * t10682;
    let t10685 = 0.76905262301422242837e-2_f64 * t2508 * t10683;
    let t10686 = t296 * t3431;
    let t10687 = t10686 * t123;
    let t10688 = t10687 * t734;
    let t10691 = t2932 * t2554;
    (t10678, t10679, t10685, t10686, t10687, t10688, t10691)
}
