//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1122/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1122(t10768: f64, t8129: f64, t2196: f64, t24790: f64, t3308: f64, t2604: f64, t625: f64, t37637: f64, t1060: f64, t269: f64, t783: f64, t7916: f64) -> (f64, f64, f64, f64, f64) {
    let t39464 = t10768 * t8129;
    let t39467 = t2196 * t3308 * t24790;
    let t39469 = t2604 * t625;
    let t39470 = t37637 * t39469;
    let t39476 = t783 * t7916 * t269 * t1060;
    (t39464, t39467, t39469, t39470, t39476)
}
