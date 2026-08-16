//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 814/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk814(t83: f64, t84: f64, t85: f64, t24: f64, t2241: f64, t645: f64, t2307: f64, t607: f64, t65: f64, t67: f64, t1864: f64, t2250: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9238 = 1.0_f64 / t85 / t84 / t83;
    let t9239 = t24 * t9238;
    let t9240 = t2241 * t645;
    let t9243 = t645 * t2307;
    let t9247 = t607 * t65 * t67;
    let t9248 = t1864 * t2250;
    (t9238, t9239, t9240, t9243, t9247, t9248)
}
