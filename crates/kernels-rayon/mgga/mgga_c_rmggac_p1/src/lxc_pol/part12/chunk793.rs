//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 793/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk793(t36801: f64, t2019: f64, t2165: f64, t7926: f64, t7328: f64, t7487: f64, t2169: f64, t7331: f64, t2020: f64, t7216: f64, t7244: f64, t7371: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36802 = 0.5854811038705731867e-3_f64 * t36801;
    let t36804 = t2019 * t7926 * t2165;
    let t36806 = t7487 * t7328;
    let t36809 = t2019 * t7926 * t2169;
    let t36811 = t7487 * t7331;
    let t36814 = t2019 * t2020 * t7216;
    let t36860 = t7244 * t7371;
    (t36802, t36804, t36806, t36809, t36811, t36814, t36860)
}
