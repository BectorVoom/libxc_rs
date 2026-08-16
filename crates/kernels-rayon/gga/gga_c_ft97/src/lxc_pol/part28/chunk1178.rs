//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1178/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1178(t165: f64, t34918: f64, t1349: f64, t34970: f64, t376: f64, t35011: f64, t34979: f64, t104289: f64, t138433: f64, t138677: f64, t138681: f64, t138705: f64, t1969: f64, t24080: f64, t26791: f64, t27411: f64, t28: f64, t33000: f64, t3424: f64, t34961: f64, t379: f64, t5772: f64, t5778: f64, t614: f64, t95403: f64) -> f64 {
    let t149347 = t34918 * t165;
    let t149357 = t1349 * t376 * t34970;
    let t149360 = t1349 * t376 * t35011;
    let t149363 = t1349 * t376 * t34979;
    let t149369 = -2.0_f64 / 3.0_f64 * t1349 * t28 * t26791 * t33000 - t138677 / 18.0_f64 - 24.0_f64 * t95403 * t27411 - 2.0_f64 / 3.0_f64 * t1349 * t28 * t5778 * t104289 - t5772 * t1969 * t149347 * t379 / 18.0_f64 + 2.0_f64 / 9.0_f64 * t5772 * t24080 * t138433 * t3424 - t149357 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t149360 + t138681 + t138705 - t149363 / 9.0_f64 + t1349 * t28 * t34961 * t614 / 6.0_f64;
    t149369
}
