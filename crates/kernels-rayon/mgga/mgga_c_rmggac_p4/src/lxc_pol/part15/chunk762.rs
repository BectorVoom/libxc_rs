//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 762/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk762(t35716: f64, t302: f64, t7350: f64, t7349: f64, t7353: f64, t31: f64, t35214: f64, t7351: f64, t35604: f64, t7338: f64, t7345: f64, t7341: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35717 = 0.13010691197123848594e-3_f64 * t35716;
    let t35718 = t7350 * t302;
    let t35720 = t7349 * t35718 * t7353;
    let t35724 = t7349 * t7351 * t35214 * t31;
    let t35728 = t7349 * t7351 * t35604 * t31;
    let t35729 = 0.65053455985619242968e-4_f64 * t35728;
    let t35742 = t7345 * t7338;
    let t35744 = t7345 * t7341;
    (t35717, t35720, t35724, t35729, t35742, t35744)
}
