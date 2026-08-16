//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 742/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk742(t1546: f64, t4426: f64, t89: f64, t4432: f64, t1586: f64, t4495: f64, t432: f64, t28: f64, t3013: f64, t3103: f64, t1577: f64, t7743: f64) -> (f64, f64, f64, f64, f64) {
    let t15609 = t89 * t1546 * t4426;
    let t15612 = t89 * t1546 * t4432;
    let t15614 = t1586 * t4495;
    let t15615 = t15614 * t432;
    let t15617 = t89 * t28 * t15615;
    let t15619 = t3013 * t3103;
    let t15621 = t89 * t28 * t15619;
    let t15625 = -2.0_f64 * t1577 - 6.0_f64 * t7743;
    (t15609, t15612, t15617, t15621, t15625)
}
