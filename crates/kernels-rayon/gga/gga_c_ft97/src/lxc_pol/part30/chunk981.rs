//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 981/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk981(t34083: f64, t8392: f64, t1882: f64, t34095: f64, t34178: f64, t34183: f64, t34187: f64, t34246: f64, t34217: f64, t34126: f64, t7674: f64, t8232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t144140 = t8392 * t34083;
    let t144142 = t1882 * t34095;
    let t144148 = t1882 * t34178;
    let t144150 = t1882 * t34183;
    let t144153 = t1882 * t34187;
    let t144162 = t1882 * t34246;
    let t144176 = t1882 * t34217;
    let t144178 = t1882 * t34126;
    let t144184 = 8.0_f64 / 27.0_f64 * t8232 * t7674;
    (t144140, t144142, t144148, t144150, t144153, t144162, t144176, t144178, t144184)
}
