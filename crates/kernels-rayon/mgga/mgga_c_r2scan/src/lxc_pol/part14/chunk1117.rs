//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1117/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1117(t11808: f64, t37685: f64, t11811: f64, t37641: f64, t10768: f64, t8129: f64, t2196: f64, t24790: f64, t3308: f64, t2604: f64, t625: f64, t37637: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39460 = t37685 * t11808;
    let t39462 = t37641 * t11811;
    let t39464 = t10768 * t8129;
    let t39467 = t2196 * t3308 * t24790;
    let t39469 = t2604 * t625;
    let t39470 = t37637 * t39469;
    (t39460, t39462, t39464, t39467, t39469, t39470)
}
