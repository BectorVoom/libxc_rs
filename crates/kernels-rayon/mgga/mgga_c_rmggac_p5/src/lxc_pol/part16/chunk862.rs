//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 862/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk862(t38460: f64, t38559: f64, t38562: f64, t38622: f64, t38639: f64, t38643: f64, t38645: f64, t38647: f64, t38675: f64, t38704: f64, t38710: f64, t38712: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42621 = 0.11173207471990682842e-3_f64 * t38460;
    let t42665 = 0.162600798888400151e-2_f64 * t38559;
    let t42666 = 0.162600798888400151e-2_f64 * t38562;
    let t42685 = 0.49658699875514145965e-4_f64 * t38622;
    let t42693 = 0.39726959900411316772e-4_f64 * t38639;
    let t42696 = 0.11918087970123395032e-3_f64 * t38643;
    let t42697 = 0.11918087970123395032e-3_f64 * t38645;
    let t42698 = 0.39726959900411316772e-4_f64 * t38647;
    let t42702 = 0.15965655602485078085e0_f64 * t38675;
    let t42712 = 0.35754263910370185096e-3_f64 * t38704;
    let t42714 = 0.47672351880493580128e-3_f64 * t38710;
    let t42715 = 0.11918087970123395032e-3_f64 * t38712;
    (t42621, t42665, t42666, t42685, t42693, t42696, t42697, t42698, t42702, t42712, t42714, t42715)
}
