//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 440/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk440(t4709: f64, t4728: f64, t1297: f64, t1314: f64, t1302: f64, t1310: f64, t20: f64, t252: f64, t43: f64, t1303: f64, t1309: f64, t239: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4729 = t4728 * t4709;
    let t4732 = t1297 * t1314;
    let t4737 = t1310 * t1302;
    let t4738 = t252 * t20;
    let t4739 = t4738 * t43;
    let t4742 = t1303 * t1314;
    let t4746 = 1.0_f64 / t1309 / t239;
    (t4729, t4732, t4737, t4739, t4742, t4746)
}
