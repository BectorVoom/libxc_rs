//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1046/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1046(t238: f64, t41588: f64, t41635: f64, t41681: f64, t41791: f64, t27: f64, t676: f64, t89: f64, t1636: f64, t2460: f64, t375: f64, t9693: f64, t2999: f64, t714: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t239 = 0.1e-59_f64 < t238;
    let t41794 = piecewise3(t239, t41588 + t41635 + t41681 + t41791, 0.0_f64);
    let t41797 = t89 * t27 * t676 * t41794;
    let t41800 = t89 * t1636 * t2460;
    let t41801 = 4.0_f64 / 9.0_f64 * t41800;
    let t41803 = t89 * t375 * t9693;
    let t41806 = t89 * t2999 * t714;
    (t41794, t41797, t41800, t41801, t41803, t41806)
}
