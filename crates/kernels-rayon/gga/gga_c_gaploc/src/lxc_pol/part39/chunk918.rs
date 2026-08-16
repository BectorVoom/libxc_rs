//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 918/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk918(t41809: f64, t447: f64, t6963: f64, t6964: f64, t12915: f64, t4953: f64, t1445: f64, t1562: f64, t34202: f64, t874: f64, t34157: f64, t895: f64) -> (f64, f64, f64, f64, f64) {
    let t41810 = t41809 * t447;
    let t41813 = 0.71500979903700853338e0_f64 * t6963 * t6964 * t41810;
    let t41814 = t4953 * t12915;
    let t41818 = t1562 * t1445 * t34202 * t874;
    let t41820 = t895 * t34157;
    (t41810, t41813, t41814, t41818, t41820)
}
