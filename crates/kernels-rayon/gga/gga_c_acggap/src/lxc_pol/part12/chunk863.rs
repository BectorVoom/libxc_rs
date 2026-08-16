//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 863/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk863(t30120: f64, t7836: f64, t3427: f64, t7647: f64, t7419: f64, t7839: f64, t1530: f64, t7584: f64, t1992: f64, t7842: f64, t945: f64, t7580: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30121 = t30120 * t7836;
    let t30123 = t7647 * t3427;
    let t30125 = t7839 * t7419;
    let t30127 = t1530 * t7584;
    let t30130 = t30127 * t7842 * t1992 * t945;
    let t30132 = t7839 * t7580;
    (t30121, t30123, t30125, t30127, t30130, t30132)
}
