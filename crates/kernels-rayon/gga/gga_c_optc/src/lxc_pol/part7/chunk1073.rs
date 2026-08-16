//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1073/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1073(t104: f64, t1928: f64, t22134: f64, t22136: f64, t22141: f64, t22143: f64, t22152: f64, t22274: f64, t22277: f64, t22281: f64, t23281: f64, t3539: f64, t6312: f64, t6704: f64, t714: f64, t95: f64) -> f64 {
    let t23286 = -t22134 - t22136 - 0.93041573165652349788e-1_f64 * t3539 * t6704 * t1928 + t22141 - t22143 + 0.93041573165652349788e-1_f64 * t3539 * t6312 * t1928 + t22152 + 0.25844881434903430496e-2_f64 * t95 * t104 * t23281 * t714 + t22274 + t22277 + t22281;
    t23286
}
