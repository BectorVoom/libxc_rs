//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 924/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk924(t2399: f64, t5134: f64, t89: f64, t38953: f64, t5172: f64, t5176: f64, t8232: f64, t2567: f64, t5132: f64, t737: f64, t5167: f64, t52212: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t68200 = t89 * t2399 * t5134;
    let t68220 = t38953 * t5172;
    let t68429 = t8232 * t5176;
    let t68528 = t5132 * t2567;
    let t68626 = t737 * t5132;
    let t68662 = t38953 * t5167;
    let t68751 = 56.0_f64 / 243.0_f64 * t52212;
    (t68200, t68220, t68429, t68528, t68626, t68662, t68751)
}
