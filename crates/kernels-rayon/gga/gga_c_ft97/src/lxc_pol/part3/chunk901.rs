//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 901/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk901(t17899: f64, t2379: f64, t1096: f64, t3750: f64, t25: f64, t5049: f64, t3762: f64, t5025: f64, t5005: f64, t1113: f64, t3751: f64, t3725: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17900 = t2379 * t17899;
    let t17903 = t1096 * t3750;
    let t17904 = t2379 * t17903;
    let t17907 = t5049 * t25;
    let t17908 = t17907 * t3762;
    let t17911 = t5025 * t25;
    let t17912 = t17911 * t3762;
    let t17915 = t5005 * t25;
    let t17916 = t17915 * t3762;
    let t17919 = t3751 * t1113;
    let t17923 = t3725 * t1113;
    (t17900, t17903, t17904, t17908, t17912, t17916, t17919, t17923)
}
