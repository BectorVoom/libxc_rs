//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1087/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1087(t1782: f64, t355: f64, t1980: f64, t5011: f64, t7458: f64, t2001: f64, t5821: f64, t1998: f64, t5569: f64, t1967: f64, t9554: f64, t6161: f64, t7561: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39066 = t355 * t1782;
    let t39069 = t1980 * t7458 * t5011 * t39066;
    let t39071 = t2001 * t5821;
    let t39073 = t1998 * t5569;
    let t39075 = t1967 * t9554;
    let t39077 = t7561 * t6161;
    (t39066, t39069, t39071, t39073, t39075, t39077)
}
