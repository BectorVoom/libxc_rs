//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1209/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1209(t1294: f64, t9919: f64, t2663: f64, t3814: f64, t9905: f64, t9892: f64, t3826: f64, t588: f64, t3684: f64, t9467: f64, t118: f64, t1284: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12094 = 0.35089341735807877242e1_f64 * t1294 * t9919;
    let t12097 = t3814 * t2663;
    let t12103 = 0.35089341735807877242e1_f64 * t1294 * t9905;
    let t12105 = 0.51947577317044391277e2_f64 * t1294 * t9892;
    let t12106 = t588 * t3826;
    let t12109 = 0.21687162600603479684e-1_f64 * t3684 * t9467;
    let t12110 = t1284 * t118;
    (t12094, t12097, t12103, t12105, t12106, t12109, t12110)
}
