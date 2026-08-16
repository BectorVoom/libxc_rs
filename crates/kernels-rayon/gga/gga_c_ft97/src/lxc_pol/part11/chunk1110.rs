//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1110/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1110(t2956: f64, t10818: f64, t5: f64, t2832: f64, t2842: f64, t2844: f64, t2801: f64, t2843: f64, t10799: f64, t875: f64, t2857: f64, t41473: f64, t446: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43304 = t2956 * t2956;
    let t43311 = t5 * t10818;
    let t43328 = t2832 * t2842;
    let t43329 = t43328 * t2844;
    let t43331 = t2801 * t2801;
    let t43332 = t2843 * t43331;
    let t43335 = t2843 * t875 * t10799;
    let t43348 = t446 * t2857 * t41473;
    (t43304, t43311, t43329, t43332, t43335, t43348)
}
