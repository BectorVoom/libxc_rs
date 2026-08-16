//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 890/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk890(t3351: f64, t511: f64, t618: f64, t7231: f64, t848: f64, t515: f64, t866: f64, t2283: f64, t36542: f64, t7404: f64, t8571: f64, t1635: f64, t1971: f64, t495: f64, t7230: f64, t880: f64) -> (f64, f64, f64, f64, f64) {
    let t39726 = t3351 * t7231 * t511 * t618 * t848;
    let t39731 = t3351 * t7231 * t515 * t618 * t866;
    let t39733 = t36542 * t2283;
    let t39735 = t8571 * t7404;
    let t39742 = t7230 * t1971 * t880 * t1635 * t495;
    (t39726, t39731, t39733, t39735, t39742)
}
