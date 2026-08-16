//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1273/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1273(t29900: f64, t8139: f64, t64: f64, t9365: f64, t38: f64, t96: f64, t95: f64, t2341: f64, t91: f64, t112: f64, t8153: f64, t111: f64, t2186: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29901 = t29900 * t8139;
    let t29903 = t64 * t9365;
    let t29907 = t38 * t96;
    let t29922 = t38 * t95;
    let t29926 = t91 * t2341;
    let t29993 = t8153 * t112;
    let t29996 = t2186 * t111;
    (t29901, t29903, t29907, t29922, t29926, t29993, t29996)
}
