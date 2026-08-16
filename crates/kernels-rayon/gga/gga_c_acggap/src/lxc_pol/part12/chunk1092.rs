//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1092/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1092(t1323: f64, t361: f64, t7436: f64, t1327: f64, t8888: f64, t2060: f64, t8630: f64, t1413: f64, t7685: f64, t2001: f64, t4535: f64, t1441: f64, t7614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35439 = t7436 * t361 * t1323;
    let t35442 = t8888 * t361 * t1327;
    let t35445 = t2060 * t361 * t8630;
    let t35447 = t7685 * t1413;
    let t35449 = t2001 * t4535;
    let t35451 = t7614 * t1441;
    (t35439, t35442, t35445, t35447, t35449, t35451)
}
