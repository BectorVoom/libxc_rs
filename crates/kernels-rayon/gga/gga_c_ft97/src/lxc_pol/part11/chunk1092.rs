//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1092/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1092(t1882: f64, t9816: f64, t10026: f64, t9993: f64, t10046: f64, t10041: f64, t756: f64, t89: f64, t9555: f64, t2587: f64, t8232: f64, t8392: f64, t9799: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42785 = t1882 * t9816;
    let t42795 = t1882 * t10026;
    let t42805 = t1882 * t9993;
    let t42807 = t1882 * t10046;
    let t42809 = t1882 * t10041;
    let t42812 = t89 * t9555 * t756;
    let t42819 = t8232 * t2587;
    let t42832 = t8392 * t9799;
    (t42785, t42795, t42805, t42807, t42809, t42812, t42819, t42832)
}
