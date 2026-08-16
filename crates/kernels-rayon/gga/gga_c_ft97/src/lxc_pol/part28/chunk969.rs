//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 969/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk969(t7235: f64, t8232: f64, t7271: f64, t32490: f64, t8392: f64, t463: f64, t7264: f64, t1882: f64, t32577: f64, t487: f64, t7165: f64, t1637: f64, t7266: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t138288 = 4.0_f64 / 27.0_f64 * t8232 * t7235;
    let t138290 = 8.0_f64 / 27.0_f64 * t8232 * t7271;
    let t138296 = t8392 * t32490;
    let t138298 = t463 * t7264;
    let t138302 = t1882 * t32577;
    let t138307 = t487 * t7165;
    let t138361 = 4.0_f64 / 27.0_f64 * t89 * t1637 * t7266;
    (t138288, t138290, t138296, t138298, t138302, t138307, t138361)
}
