//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1007/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1007(t8392: f64, t9146: f64, t599: f64, t7943: f64, t89: f64, t1882: f64, t9412: f64, t9318: f64, t9313: f64, t9306: f64, t161: f64, t38061: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41047 = t8392 * t9146;
    let t41050 = t89 * t7943 * t599;
    let t41064 = t1882 * t9412;
    let t41074 = t1882 * t9318;
    let t41076 = t1882 * t9313;
    let t41084 = t1882 * t9306;
    let t41093 = 280.0_f64 / 243.0_f64 * t89 * t38061 * t161;
    (t41047, t41050, t41064, t41074, t41076, t41084, t41093)
}
