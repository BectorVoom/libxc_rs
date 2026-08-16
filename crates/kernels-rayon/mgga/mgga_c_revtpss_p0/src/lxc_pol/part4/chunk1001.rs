//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1001/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1001(t10547: f64, t2798: f64, t760: f64, t9323: f64, t9318: f64, t2251: f64, t750: f64, t2611: f64, t2398: f64, t2615: f64, t2609: f64, t717: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10548 = t2798 * t10547;
    let t10552 = 0.51947577317044391277e2_f64 * t760 * t9323;
    let t10554 = 0.35089341735807877242e1_f64 * t760 * t9318;
    let t10555 = t750 * t2251;
    let t10556 = t2611 * t10555;
    let t10561 = t2398 * t2615;
    let t10563 = t717 * t2609;
    (t10548, t10552, t10554, t10556, t10561, t10563)
}
