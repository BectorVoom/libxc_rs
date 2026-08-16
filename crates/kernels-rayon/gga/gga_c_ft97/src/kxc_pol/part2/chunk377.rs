//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 377/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk377(t1995: f64, t539: f64, t135: f64, t527: f64, t538: f64, t549: f64, t554: f64, t118: f64, t29: f64, t1595: f64, t120: f64, t1655: f64, t528: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1996 = t1995 * t539;
    let t2001 = t527 * t135;
    let t2002 = t549 * t538;
    let t2003 = t2002 * t554;
    let t2007 = 1.0_f64 / t118 / t29;
    let t2008 = t2007 * t1595;
    let t2009 = t2008 * t120;
    let t2011 = t528 * t1655;
    (t1996, t2001, t2003, t2007, t2008, t2009, t2011)
}
