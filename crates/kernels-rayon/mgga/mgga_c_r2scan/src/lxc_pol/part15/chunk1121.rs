//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1121/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1121(t1577: f64, t3308: f64, t7434: f64, t6218: f64, t7513: f64, t10772: f64, t10810: f64, t2568: f64, t11808: f64, t37685: f64, t11811: f64, t37641: f64) -> (f64, f64, f64, f64, f64) {
    let t39452 = t1577 * t3308 * t7434;
    let t39455 = t6218 * t3308 * t7513;
    let t39458 = t10772 * t10810 * t2568;
    let t39459 = 0.69345773920434148506e0_f64 * t39458;
    let t39460 = t37685 * t11808;
    let t39462 = t37641 * t11811;
    (t39452, t39455, t39459, t39460, t39462)
}
