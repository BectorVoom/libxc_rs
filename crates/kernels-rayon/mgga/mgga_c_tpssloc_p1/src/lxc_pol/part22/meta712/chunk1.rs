//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2311/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2311(t152: f64, t20825: f64, t607: f64, t41284: f64, t46302: f64, t20742: f64, t67: f64, t758: f64, t58047: f64, t58052: f64, t58057: f64, t40794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t67204 = 24.0_f64 * t607 * t152 * t20825;
    let t67206 = 24.0_f64 * t41284 * t20825;
    let t67207 = 0.31168546390226634765e3_f64 * t46302;
    let t67209 = t20742 * t67 * t758;
    let t67210 = 0.18311447306006545054e-3_f64 * t67209;
    let t67211 = 12.0_f64 * t58047;
    let t67212 = 24.0_f64 * t58052;
    let t67214 = 0.35089341735807877242e1_f64 * t58057;
    let t67215 = 0.16265371950452609763e-1_f64 * t40794;
    (t67204, t67206, t67207, t67210, t67211, t67212, t67214, t67215)
}
