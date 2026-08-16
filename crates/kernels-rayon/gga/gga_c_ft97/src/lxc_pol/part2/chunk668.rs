//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 668/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk668(t737: f64, t754: f64, t2344: f64, t675: f64, t255: f64, t1882: f64, t2471: f64, t731: f64, t8232: f64, t768: f64, t2563: f64, t2559: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9787 = t737 * t754;
    let t9802 = t2344 * t675;
    let t9803 = t9802 * t255;
    let t9813 = t1882 * t2471;
    let t9822 = t8232 * t731;
    let t9824 = t8232 * t768;
    let t9826 = t1882 * t2563;
    let t9828 = t1882 * t2559;
    (t9787, t9802, t9803, t9813, t9822, t9824, t9826, t9828)
}
