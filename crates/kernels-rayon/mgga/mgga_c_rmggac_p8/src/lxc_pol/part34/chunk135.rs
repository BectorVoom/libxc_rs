//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 135/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk135(t361: f64, t380: f64, t383: f64, t387: f64, t423: f64, t430: f64, t435: f64, t579: f64, t581: f64, t198: f64, t454: f64, t589: f64) -> (f64, f64) {
    let t592 = t361 + t380 - t383 - t387 + t579 + t423 + t581 - t430 - t435;
    let t597 = -0.32163648644302209643e2_f64 * t592 * t198 + 0.96490945932906628929e2_f64 * t454 * t589;
    (t592, t597)
}
