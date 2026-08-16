//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 802/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk802(t12609: f64, t9144: f64, t2190: f64, t3578: f64, t574: f64, t1026: f64, t8232: f64, t1882: f64, t3463: f64, t3590: f64, t379: f64, t569: f64) -> (f64, f64, f64, f64, f64) {
    let t12610 = t9144 * t12609;
    let t12614 = t574 * t3578 * t2190;
    let t12617 = t8232 * t1026;
    let t12620 = 2.0_f64 / 27.0_f64 * t1882 * t3463;
    let t12622 = t569 * t3590 * t379;
    (t12610, t12614, t12617, t12620, t12622)
}
