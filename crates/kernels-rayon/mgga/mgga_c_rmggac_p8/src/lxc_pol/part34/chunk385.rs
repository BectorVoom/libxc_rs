//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 385/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk385(t128: f64, t209: f64, t476: f64, t118: f64, t2106: f64, t261: f64, t1297: f64, t20: f64, t2018: f64, t511: f64, t892: f64, t504: f64, t880: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7476 = t128 * t476 * t209;
    let t7477 = t118 * t7476;
    let t7487 = t261 * t2106;
    let t7490 = t1297 * t20;
    let t7491 = t7490 * t2018;
    let t7494 = t892 * t511;
    let t7501 = t504 * t880;
    (t7476, t7477, t7487, t7490, t7491, t7494, t7501)
}
