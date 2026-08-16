//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 419/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk419(t1154: f64, t2475: f64, t1148: f64, t1775: f64, t2: f64, t2486: f64, t737: f64, t1152: f64, t458: f64, t1131: f64, t3688: f64, t3710: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3902 = t2475 * t1154;
    let t3908 = t1775 * t1148;
    let t3910 = t2486 * t2;
    let t3917 = t737 * t2;
    let t3925 = t458 * t1152;
    let t3930 = t2 * t1131;
    let t3942 = t3688 / 27.0_f64;
    let t3947 = t3710 / 9.0_f64;
    (t3902, t3908, t3910, t3917, t3925, t3930, t3942, t3947)
}
