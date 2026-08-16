//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 816/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk816(t1971: f64, t236: f64, t38454: f64, t5561: f64, t16155: f64, t8516: f64, t8519: f64, t615: f64, t7230: f64, t794: f64, t9188: f64, t17859: f64, t7742: f64) -> (f64, f64, f64, f64) {
    let t38457 = t38454 * t1971 * t236 * t5561;
    let t38460 = t8516 * t16155 * t8519;
    let t38465 = t7230 * t9188 * t236 * t615 * t794;
    let t38467 = t17859 * t7742;
    (t38457, t38460, t38465, t38467)
}
