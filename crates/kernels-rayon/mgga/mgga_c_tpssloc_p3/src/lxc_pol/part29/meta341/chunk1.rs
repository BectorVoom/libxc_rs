//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1402/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1402(t3681: f64, t67: f64, t758: f64, t1294: f64, t9905: f64, t9892: f64, t3684: f64, t9467: f64, t118: f64, t1284: f64, t2375: f64, t9882: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12099 = t3681 * t67;
    let t12100 = t12099 * t758;
    let t12103 = 0.35089341735807877242e1_f64 * t1294 * t9905;
    let t12105 = 0.51947577317044391277e2_f64 * t1294 * t9892;
    let t12109 = 0.21687162600603479684e-1_f64 * t3684 * t9467;
    let t12110 = t1284 * t118;
    let t12111 = t12110 * t2375;
    let t12114 = 0.32530743900905219526e-1_f64 * t3684 * t9882;
    (t12100, t12103, t12105, t12109, t12111, t12114)
}
