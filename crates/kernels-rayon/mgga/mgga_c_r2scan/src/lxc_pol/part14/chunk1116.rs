//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1116/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1116(t11816: f64, t37880: f64, t3308: f64, t6449: f64, t7462: f64, t1577: f64, t7434: f64, t6218: f64, t7513: f64, t10772: f64, t10810: f64, t2568: f64) -> (f64, f64, f64, f64, f64) {
    let t39445 = t37880 * t11816;
    let t39448 = t6449 * t3308 * t7462;
    let t39452 = t1577 * t3308 * t7434;
    let t39455 = t6218 * t3308 * t7513;
    let t39458 = t10772 * t10810 * t2568;
    (t39445, t39448, t39452, t39455, t39458)
}
