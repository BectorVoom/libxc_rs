//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1123/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1123(t10717: f64, t684: f64, t10248: f64, t446: f64, t2739: f64, t2681: f64, t27: f64, t89: f64, t1636: f64, t2683: f64, t10263: f64, t375: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43509 = t684 * t10717;
    let t43511 = t446 * t10248 * t43509;
    let t43513 = t2739 * t2739;
    let t43516 = t89 * t27 * t2681 * t43513;
    let t43519 = t89 * t1636 * t2683;
    let t43522 = t89 * t375 * t10263;
    (t43509, t43511, t43513, t43516, t43519, t43522)
}
