//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 861/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk861(t732: f64, t9242: f64, t3625: f64, t723: f64, t730: f64, t179: f64, t780: f64, t9161: f64, t1123: f64, t2003: f64, t300: f64, t2774: f64, t761: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9244 = 0.5848223622634646207e0_f64 * t9242 * t732;
    let t9245 = t3625 * t723;
    let t9247 = 0.35089341735807877242e1_f64 * t730 * t9245;
    let t9253 = t179 * t780 * t9161;
    let t9257 = t2003 * t1123;
    let t9258 = t300 * t9257;
    let t9259 = t761 * t2774;
    (t9244, t9245, t9247, t9253, t9257, t9258, t9259)
}
