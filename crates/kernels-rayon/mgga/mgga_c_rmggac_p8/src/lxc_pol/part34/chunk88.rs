//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 88/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk88(t84: f64, t280: f64, t90: f64, t101: f64, t266: f64, t87: f64, t91: f64, t98: f64, rho0: f64, tau0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t306 = rho0 * rho0;
    let t308 = 1.0_f64 / t84 / t306;
    let t309 = tau0 * t308;
    let t312 = t280 / 2.0_f64;
    let t313 = t90 * t312;
    let t316 = -t312;
    let t317 = t101 * t316;
    let t320 = 2.0_f64 / 3.0_f64 * t266;
    let t321 = -10.0_f64 / 3.0_f64 * t309 * t91 + 10.0_f64 / 3.0_f64 * t87 * t313 + 10.0_f64 / 3.0_f64 * t98 * t317 + t320;
    (t309, t312, t316, t317, t320, t321)
}
