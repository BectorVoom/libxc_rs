//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 880/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk880(t1184: f64, t12930: f64, t1190: f64, t182: f64, t862: f64, t1162: f64) -> (f64, f64, f64, f64) {
    let t12931 = t12930 * t1184;
    let t12933 = t12930 * t1190;
    let t12935 = t862 * t182;
    let t12936 = t12935 * t1162;
    (t12931, t12933, t12935, t12936)
}
