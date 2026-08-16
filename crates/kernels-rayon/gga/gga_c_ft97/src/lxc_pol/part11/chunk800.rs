//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 800/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk800(t10818: f64, t332: f64, t1934: f64, t910: f64, t2899: f64, t5: f64, t2957: f64, t505: f64, t2253: f64, t2953: f64, t170: f64, t328: f64, t8715: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10819 = t10818 * t332;
    let t10823 = t910 * t1934;
    let t10829 = t5 * t2899;
    let t10832 = t2957 * t505;
    let t10835 = t2253 * t2953;
    let t10838 = 20.0_f64 / 27.0_f64 * t170 * t8715 * t328;
    (t10819, t10823, t10829, t10832, t10835, t10838)
}
