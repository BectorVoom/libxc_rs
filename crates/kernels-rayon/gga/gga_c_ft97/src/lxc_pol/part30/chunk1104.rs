//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1104/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1104(t10248: f64, t152673: f64, t446: f64, t152678: f64, t43350: f64, t143284: f64, t3886: f64, t2665: f64, t24980: f64, t24981: f64, t28729: f64, t33978: f64) -> (f64, f64, f64, f64, f64) {
    let t152810 = t446 * t10248 * t152673;
    let t152813 = t446 * t43350 * t152678;
    let t152815 = t143284 * t3886;
    let t152817 = t446 * t2665 * t152815;
    let t152821 = t24980 * t24981 * t33978 * t28729;
    (t152810, t152813, t152815, t152817, t152821)
}
