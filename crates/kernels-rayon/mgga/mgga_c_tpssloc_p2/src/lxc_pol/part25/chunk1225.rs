//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1225/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1225(t2105: f64, t3931: f64, t1404: f64, t7222: f64, t24447: f64, t580: f64, t2098: f64, t3946: f64, t1395: f64, t7240: f64, t12513: f64, t12537: f64, t1396: f64, t1398: f64, t2099: f64, t24448: f64, t24486: f64, t3: f64, t3932: f64, t7223: f64, t84031: f64, t85372: f64, t85375: f64) -> f64 {
    let t85379 = t3931 * t2105;
    let t85381 = t7222 * t1404;
    let t85392 = t24447 * t580;
    let t85394 = t2098 * t3946;
    let t85397 = t1395 * t7240;
    let tv4rho3sigma1 = t3 * t580 * t85372 + t12513 * t2105 + t12537 * t2099 + 3.0_f64 * t1396 * t24486 + t1398 * t85375 + 3.0_f64 * t1404 * t24448 + 3.0_f64 * t3932 * t7240 + 3.0_f64 * t3946 * t7223 + 3.0_f64 * t84031 + 3.0_f64 * t85379 + 6.0_f64 * t85381 + 3.0_f64 * t85392 + 3.0_f64 * t85394 + 6.0_f64 * t85397;
    tv4rho3sigma1
}
