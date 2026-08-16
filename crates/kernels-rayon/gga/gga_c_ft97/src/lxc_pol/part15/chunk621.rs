//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 621/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk621(t1160: f64, t737: f64, t2567: f64, t668: f64, t1144: f64, t8232: f64, t2372: f64, t255: f64, t1131: f64, t761: f64, t13722: f64, t13739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13839 = t737 * t1160;
    let t13857 = t2567 * t668;
    let t13872 = t8232 * t1144;
    let t13885 = t2372 * t255;
    let t13886 = t761 * t1131;
    let t13927 = t1160 * t2567;
    let t13976 = 4.0_f64 / 27.0_f64 * t13722;
    let t13981 = 4.0_f64 / 9.0_f64 * t13739;
    (t13839, t13857, t13872, t13885, t13886, t13927, t13976, t13981)
}
