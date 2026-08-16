//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 888/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk888(t2: f64, t33828: f64, t798: f64, t9567: f64, t295: f64, t41751: f64, t665: f64, t7640: f64, t2344: f64, t2680: f64, t309: f64, t43537: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43803 = t33828 * t2;
    let t43833 = t9567 * t798;
    let t43834 = t43833 * t2;
    let t43852 = t41751 * t295;
    let t43912 = t665 * t7640;
    let t43913 = t43912 * t2;
    let t43917 = t2344 * t2680;
    let t43918 = t43917 * t2;
    let t44042 = t43912 * t309;
    let t44121 = 280.0_f64 / 81.0_f64 * t43537;
    (t43803, t43833, t43834, t43852, t43913, t43917, t43918, t44042, t44121)
}
