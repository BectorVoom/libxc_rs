//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2286/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2286(t17624: f64, t6717: f64, t1933: f64, t1937: f64, t5398: f64, t1022: f64, t10475: f64, t17738: f64, t23422: f64, t23678: f64, t25609: f64, t25652: f64, t25653: f64, t25654: f64, t28578: f64, t3128: f64, t4649: f64, t5866: f64, t5872: f64, t5885: f64, t7574: f64, t7583: f64, t82516: f64, t82542: f64, t82911: f64, t88286: f64, t88415: f64, t88537: f64) -> f64 {
    let t99624 = t6717 * t17624;
    let t99631 = t1933 * t5398 * t1937;
    let t99635 = 0.20186378047070195428e-3_f64 * t25652 * t3128 * t5866 * t25654 + 0.40372756094140390856e-3_f64 * t25652 * t25653 * t23678 * t4649 - 0.20186378047070195428e-3_f64 * t82911 * t28578 + 0.60559134141210586284e-3_f64 * t88537 * t10475 * t5872 * t82516 * t1022 - 0.60559134141210586284e-3_f64 * t88537 * t3128 * t5872 * t82542 * t1022 + t23422 * t5885 / 54.0_f64 - t99624 / 432.0_f64 - t88415 - 0.16149102437656156342e-2_f64 * t88286 * t7583 - 0.20186378047070195428e-3_f64 * t7574 * t25609 + 0.10093189023535097714e-3_f64 * t99631 + t6717 * t17738 / 288.0_f64;
    t99635
}
