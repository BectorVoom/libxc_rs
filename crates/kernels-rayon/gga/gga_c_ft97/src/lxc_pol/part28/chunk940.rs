//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 940/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk940(t32240: f64, t378: f64, t32156: f64, t66: f64, t1669: f64, t32168: f64, t173: f64, t32260: f64, t22819: f64, t7195: f64, t1293: f64, t37: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t136475 = t32240 * t378;
    let t136485 = t32156 * t66;
    let t136488 = t1669 * t32168;
    let t136505 = t173 * t32260;
    let t136507 = t22819 * t7195 * t136505;
    let t136516 = t37 * t1293;
    (t136475, t136485, t136488, t136505, t136507, t136516)
}
