//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 561/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk561(t348: f64, t4500: f64, t965: f64, t1766: f64, t91: f64, t1781: f64, t4417: f64, t1780: f64, t1787: f64, t4422: f64, t1791: f64, t463: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4501 = t348 * t4500;
    let t4505 = t965 * t965;
    let t4507 = t91 * t1766 * t4505;
    let t4511 = t1781 * t4417;
    let t4512 = t1780 * t4511;
    let t4515 = t1787 * t4422;
    let t4518 = t1791 * t4417;
    let t4519 = t463 * t4518;
    (t4501, t4505, t4507, t4511, t4512, t4515, t4518, t4519)
}
