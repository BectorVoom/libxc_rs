//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 962/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk962(t9484: f64, t9543: f64, t520: f64, t512: f64, t1331: f64, t3857: f64, t2619: f64, t3825: f64, t1333: f64, t3863: f64, t2626: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9544 = t9484 + t9543;
    let t9545 = t520 * t9544;
    let t9546 = t512 * t9545;
    let t9559 = t3857 * t1331;
    let t9566 = t3825 * t2619;
    let t9569 = 60.0_f64 * t3857 * t1333;
    let t9570 = t3863 * t1331;
    let t9572 = t676 * t2626;
    (t9544, t9546, t9559, t9566, t9569, t9570, t9572)
}
