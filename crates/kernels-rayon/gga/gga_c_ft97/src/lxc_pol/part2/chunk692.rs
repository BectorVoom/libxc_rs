//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 692/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk692(t10989: f64, t446: f64, t1882: f64, t3010: f64, t3052: f64, t432: f64, t1564: f64, t3281: f64, t1580: f64, t2992: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10990 = t446 * t10989;
    let t10992 = t1882 * t3010;
    let t10993 = t10992 / 27.0_f64;
    let t10994 = t3052 * t432;
    let t10995 = t1564 * t10994;
    let t10996 = t3281 * t10995;
    let t10998 = t2992 * t1580;
    (t10990, t10992, t10993, t10994, t10996, t10998)
}
