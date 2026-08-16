//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1017/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1017(t2299: f64, t13255: f64, t1580: f64, t184: f64, t21: f64, t2240: f64, t2300: f64, t2305: f64, t2309: f64, t3664: f64, t40590: f64, t41364: f64, t5: f64, t623: f64, t648: f64, t8614: f64, t8722: f64, t8724: f64, t8738: f64, t8739: f64, t8744: f64, t8751: f64, t8754: f64) -> f64 {
    let t41382 = t2299 * t2299;
    let t41399 = 3.0_f64 * t8614 * t2309 + t2240 * t8724 + t5 * (t40590 + t41364) * t184 * t21 / 4.0_f64 + 3.0_f64 / 2.0_f64 * t623 * t2305 * t21 * t2299 + 3.0_f64 * t2240 * t8751 + 3.0_f64 / 2.0_f64 * t623 * t2305 * t1580 + t623 * t8722 * t648 * t3664 + 3.0_f64 / 4.0_f64 * t623 * t41382 * t184 * t21 + 3.0_f64 * t2240 * t8739 + 3.0_f64 * t623 * t8738 * t13255 + 3.0_f64 * t2240 * t8744 + 3.0_f64 / 2.0_f64 * t623 * t2300 * t1580 + 3.0_f64 * t2240 * t8754;
    t41399
}
