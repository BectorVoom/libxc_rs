//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 894/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk894(t3775: f64, t5001: f64, t1689: f64, t236: f64, t39: f64, t3771: f64, t1613: f64, t5009: f64, t5014: f64, t3751: f64, t6: f64, t13411: f64, t688: f64) -> (f64, f64, f64, f64, f64) {
    let t17801 = t3775 * t5001;
    let t17806 = t236 * t39 * t1689;
    let t17807 = t3771 * t17806;
    let t17808 = t1613 * t5009;
    let t17809 = t17808 * t5014;
    let t17813 = t3751 * t6;
    let t17817 = t13411 * t688;
    (t17801, t17807, t17809, t17813, t17817)
}
