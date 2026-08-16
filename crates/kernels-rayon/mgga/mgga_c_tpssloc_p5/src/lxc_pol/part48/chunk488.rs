//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 488/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk488(t28: f64, t1081: f64, t3231: f64, t3672: f64, t517: f64, t157: f64, t3671: f64, t182: f64, t118: f64, t521: f64, t2375: f64, t1294: f64, t2371: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t3673 = t1081 * t1081;
    let t3679 = piecewise3(t29, 0.0_f64, 4.0_f64 / 9.0_f64 * t3672 * t3673 + 4.0_f64 / 3.0_f64 * t517 * t3231);
    let t3681 = (t3671 + t3679) * t157;
    let t3683 = 0.19751673498613801407e-1_f64 * t3681 * t182;
    let t3684 = t521 * t118;
    let t3686 = 0.10843581300301739842e-1_f64 * t3684 * t2375;
    let t3688 = 0.11696447245269292414e1_f64 * t1294 * t2371;
    (t3673, t3681, t3683, t3686, t3688)
}
