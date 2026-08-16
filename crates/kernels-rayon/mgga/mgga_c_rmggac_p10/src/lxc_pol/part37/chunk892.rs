//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 892/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk892(t623: f64, t7190: f64, t13841: f64, t70225: f64, t14063: f64, t2411: f64, t3151: f64, t13823: f64, t8465: f64, t938: f64, t15205: f64, t68581: f64) -> (f64, f64, f64, f64, f64) {
    let t75999 = t623 * t7190;
    let t76000 = t75999 * t13841;
    let t76002 = 0.15965655602485078085e0_f64 * t70225;
    let t76017 = t2411 * t14063 * t3151;
    let t76021 = t13823 * t8465 * t938;
    let t76025 = t68581 * t15205;
    (t76000, t76002, t76017, t76021, t76025)
}
