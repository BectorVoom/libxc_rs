//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 619/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk619(t132: f64, t388: f64, t7934: f64, t7933: f64, t2185: f64, t2190: f64) -> (f64, f64, f64, f64) {
    let t7935 = t388 * t132;
    let t7936 = t7934 * t7935;
    let t7937 = t7933 * t7936;
    let t7939 = t2190 * t2185;
    (t7935, t7936, t7937, t7939)
}
