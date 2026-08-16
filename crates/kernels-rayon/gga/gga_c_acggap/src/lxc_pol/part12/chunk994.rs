//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 994/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk994(t545: f64, t7923: f64, t1411: f64, t463: f64, t309: f64, t1264: f64, t525: f64, t33428: f64, t615: f64, t8396: f64, t862: f64, t556: f64, t943: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33524 = t7923 * t545;
    let t33547 = t1411 * t463;
    let t33551 = t1411 * t309;
    let t33561 = t525 * t1264;
    let t33566 = t615 * t33428;
    let t33574 = t862 * t8396;
    let t33643 = t556 * t943;
    (t33524, t33547, t33551, t33561, t33566, t33574, t33643)
}
