//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 765/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk765(t31: f64, t35604: f64, t7349: f64, t7351: f64, t2019: f64, t2020: f64, t7220: f64, t7224: f64, t7338: f64, t7345: f64, t7341: f64, t4905: f64, t7778: f64, t903: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35728 = t7349 * t7351 * t35604 * t31;
    let t35729 = 0.65053455985619242968e-4_f64 * t35728;
    let t35731 = t2019 * t2020 * t7220;
    let t35737 = t2019 * t2020 * t7224;
    let t35742 = t7345 * t7338;
    let t35744 = t7345 * t7341;
    let t35752 = t903 * t7778 * t4905;
    (t35729, t35731, t35737, t35742, t35744, t35752)
}
