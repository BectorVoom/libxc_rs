//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 661/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk661(t109: f64, t24890: f64, t5011: f64, t511: f64, t534: f64, t7350: f64, t4617: f64, t507: f64, t338: f64, t6444: f64, t26: f64, t7834: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40167 = t24890 * t109;
    let t40193 = t5011 * t511;
    let t40717 = t7350 * t534;
    let t40724 = t507 * t4617;
    let t40826 = t6444 * t338;
    let t40927 = t7834 * t26;
    (t40167, t40193, t40717, t40724, t40826, t40927)
}
