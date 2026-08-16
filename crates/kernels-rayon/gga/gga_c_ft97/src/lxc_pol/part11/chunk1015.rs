//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1015/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1015(t1526: f64, t7705: f64, t8775: f64, t15567: f64, t16633: f64, t3088: f64, t41332: f64, t41335: f64, t41338: f64, t41341: f64, t41344: f64, t41349: f64, t7765: f64, t7807: f64, t8788: f64, t8790: f64, t9050: f64) -> f64 {
    let t41358 = t1526 * t7705 * t8775;
    let t41360 = 2.0_f64 * t8790 + t41332 / 18.0_f64 - t41335 / 6.0_f64 - t41338 / 12.0_f64 - t41341 / 9.0_f64 + t8788 - t41344 / 4.0_f64 - t1526 * t3088 * t9050 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t1526 * t3088 * t41349 * t7765 - t15567 * t16633 * t7807 / 3.0_f64 + t41358 / 6.0_f64;
    t41360
}
