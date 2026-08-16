//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 587/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk587(t184: f64, t4844: f64, t2258: f64, t2259: f64, t4417: f64, t1073: f64, t2266: f64, t925: f64, t2271: f64, t72: f64, t4431: f64, t632: f64) -> (f64, f64, f64, f64, f64) {
    let t4845 = t4844 * t184;
    let t4857 = t2258 * t2259 * t4417;
    let t4861 = t2266 * t925 * t1073;
    let t4865 = t72 * t2271 * t4417;
    let t4869 = t72 * t632 * t4431;
    (t4845, t4857, t4861, t4865, t4869)
}
