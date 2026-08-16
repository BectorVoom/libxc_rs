//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 991/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk991(t10535: f64, t10538: f64, t2783: f64, t860: f64, t786: f64, t760: f64, t9323: f64, t9318: f64, t2609: f64, t717: f64, t162: f64, t9544: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10539 = t10535 * t10538;
    let t10541 = t2783 * t860;
    let t10542 = t786 * t10541;
    let t10552 = 0.51947577317044391277e2_f64 * t760 * t9323;
    let t10554 = 0.35089341735807877242e1_f64 * t760 * t9318;
    let t10563 = t717 * t2609;
    let t10565 = t162 * t9544;
    (t10539, t10542, t10552, t10554, t10563, t10565)
}
