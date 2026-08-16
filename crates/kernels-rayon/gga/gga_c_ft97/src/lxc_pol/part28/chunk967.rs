//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 967/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk967(t1882: f64, t32568: f64, t32627: f64, t7283: f64, t8232: f64, t7222: f64, t32520: f64, t32599: f64, t8392: f64, t32532: f64, t32529: f64, t32536: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t138029 = t1882 * t32568;
    let t138034 = t1882 * t32627;
    let t138057 = 4.0_f64 / 27.0_f64 * t8232 * t7283;
    let t138119 = 8.0_f64 / 27.0_f64 * t8232 * t7222;
    let t138126 = t1882 * t32520;
    let t138143 = t8392 * t32599;
    let t138154 = t1882 * t32532;
    let t138156 = t1882 * t32529;
    let t138158 = t1882 * t32536;
    (t138029, t138034, t138057, t138119, t138126, t138143, t138154, t138156, t138158)
}
