//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 987/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk987(t3928: f64, t5187: f64, t645: f64, t4044: f64, t5194: f64, t1971: f64, t236: f64, t5704: f64, t7365: f64, t35331: f64, t5700: f64, t36772: f64, t9147: f64) -> (f64, f64, f64, f64, f64) {
    let t41672 = t3928 * t645 * t5187;
    let t41675 = t4044 * t645 * t5194;
    let t41690 = t7365 * t1971 * t236 * t5704;
    let t41694 = t35331 * t1971 * t236 * t5700;
    let t41696 = t36772 * t9147;
    (t41672, t41675, t41690, t41694, t41696)
}
