//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 564/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk564(t3700: f64, t724: f64, t446: f64, t2999: f64, t665: f64, t18: f64, t669: f64, t89: f64, t1132: f64, t375: f64, t1131: f64, t668: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3701 = t724 * t3700;
    let t3702 = t446 * t3701;
    let t3704 = t2999 * t665;
    let t3705 = t669 * t18;
    let t3707 = t89 * t3704 * t3705;
    let t3710 = t89 * t375 * t1132;
    let t3712 = t1131 * t668;
    (t3701, t3702, t3704, t3705, t3707, t3710, t3712)
}
