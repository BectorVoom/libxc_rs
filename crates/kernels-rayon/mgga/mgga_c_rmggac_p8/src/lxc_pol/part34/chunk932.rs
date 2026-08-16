//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 932/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk932(t73783: f64, t73746: f64, t73749: f64, t73764: f64, t73787: f64, t76628: f64, t76631: f64, t76632: f64, t76633: f64, t76634: f64, t76635: f64, t76637: f64, t76638: f64, t76639: f64, t76640: f64, t76641: f64, t76642: f64) -> f64 {
    let t76643 = 0.19709219354514038085e-5_f64 * t73783;
    let t76645 = -t76628 - 0.4379826523225341797e-6_f64 * t73746 - 0.35038612185802734376e-6_f64 * t73749 - t76631 + t76632 - t76633 - t76634 - t76635 - 0.52557918278704101564e-6_f64 * t73764 + t76637 - t76638 - t76639 + t76640 - t76641 - t76642 + t76643 - 0.87596530464506835935e-6_f64 * t73787;
    t76645
}
