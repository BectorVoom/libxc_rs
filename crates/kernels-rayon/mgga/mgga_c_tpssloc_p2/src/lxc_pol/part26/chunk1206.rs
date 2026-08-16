//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1206/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1206(t12022: f64, t12027: f64, t1375: f64, t1386: f64, t2015: f64, t2016: f64, t22653: f64, t3882: f64, t39910: f64, t40591: f64, t6958: f64, t80671: f64, t80675: f64, t80678: f64, t80683: f64, t80687: f64, t80689: f64, t80699: f64) -> f64 {
    let t80702 = -0.15626873635058151147e0_f64 * t80671 - 0.82246703342411321825e-2_f64 * t80675 + 0.14804406601634037928e0_f64 * t80678 - 0.74022033008170189643e-1_f64 * t80683 - t39910 * t2016 - 0.24674011002723396548e-1_f64 * t80687 + 0.57572692339687925277e-1_f64 * t80689 + 6.0_f64 * t6958 * t12027 + 24.0_f64 * t1375 * t40591 * t2015 * t12022 + 12.0_f64 * t3882 * t22653 - 6.0_f64 * t80699 * t1386;
    t80702
}
