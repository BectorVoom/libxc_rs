//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 830/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk830(t10295: f64, t27214: f64, t3073: f64, t3459: f64, t5559: f64, t35781: f64, t977: f64, t11288: f64, t2497: f64, t3601: f64, t935: f64) -> (f64, f64, f64, f64, f64) {
    let t44694 = 12.0_f64 * t27214 * t10295;
    let t44697 = 12.0_f64 * t5559 * t3459 * t3073;
    let t44702 = t35781 * t977;
    let t44705 = t11288 * t2497;
    let t44707 = t3601 * t935;
    (t44694, t44697, t44702, t44705, t44707)
}
