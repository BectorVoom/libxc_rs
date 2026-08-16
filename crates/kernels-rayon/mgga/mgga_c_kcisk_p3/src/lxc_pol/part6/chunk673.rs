//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 673/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk673(t10568: f64, t5005: f64, t79: f64, t311: f64, t3841: f64, t579: f64, t571: f64, t574: f64, t581: f64, t4786: f64, t596: f64, t1675: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10569 = 0.55403703703703703703e-1_f64 * t10568;
    let t10621 = t79 * t5005;
    let t10639 = 0.93932222222222222223e0_f64 * t10568;
    let t10641 = t311 * t3841 * t579;
    let t10642 = 0.36793333333333333333e0_f64 * t10641;
    let t10649 = 28.0_f64 / 27.0_f64 * t10568;
    let t10663 = 1.0_f64/pow_3_2(t571);
    let t10671 = 1.0_f64 / t574 / t581 / 4.0_f64;
    let t10690 = 1.0_f64 / t4786 / t596;
    let t10696 = 1.0_f64 / t4786 / t1675;
    (t10569, t10621, t10639, t10641, t10642, t10649, t10663, t10671, t10690, t10696)
}
