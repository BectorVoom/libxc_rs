//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 855/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk855(t13320: f64, t3910: f64, t1091: f64, t2459: f64, t2493: f64, t1775: f64, t3914: f64, t2372: f64, t3930: f64, t1148: f64, t8282: f64, t3932: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13321 = t3910 * t13320;
    let t13324 = t1091 * t2459;
    let t13325 = t2493 * t13324;
    let t13329 = 2.0_f64 / 9.0_f64 * t1775 * t3914;
    let t13332 = t2372 * t3930 * t2459;
    let t13335 = t8282 * t1148;
    let t13338 = 4.0_f64 / 3.0_f64 * t1775 * t3932;
    (t13321, t13324, t13325, t13329, t13332, t13335, t13338)
}
