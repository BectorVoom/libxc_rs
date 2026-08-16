//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 408/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk408(t88: f64, t89: f64, t154: f64, t50: f64, t100: f64, t99: f64, t297: f64, t34: f64) -> (f64, f64, f64, f64) {
    let t3868 = t89 * t88;
    let t3869 = 1.0_f64 / t3868;
    let t3878 = t50 * t154;
    let t3884 = t100 * t99;
    let t3885 = 1.0_f64 / t3884;
    let t3899 = 1.0_f64 / t34 / t297;
    (t3869, t3878, t3885, t3899)
}
