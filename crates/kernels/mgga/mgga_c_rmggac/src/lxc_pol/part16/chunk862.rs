//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 862/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk862<F: Float>(t38460: F, t38559: F, t38562: F, t38622: F, t38639: F, t38643: F, t38645: F, t38647: F, t38675: F, t38704: F, t38710: F, t38712: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42621 = F::cast_from(0.11173207471990682842e-3_f64) * t38460;
    let t42665 = F::cast_from(0.162600798888400151e-2_f64) * t38559;
    let t42666 = F::cast_from(0.162600798888400151e-2_f64) * t38562;
    let t42685 = F::cast_from(0.49658699875514145965e-4_f64) * t38622;
    let t42693 = F::cast_from(0.39726959900411316772e-4_f64) * t38639;
    let t42696 = F::cast_from(0.11918087970123395032e-3_f64) * t38643;
    let t42697 = F::cast_from(0.11918087970123395032e-3_f64) * t38645;
    let t42698 = F::cast_from(0.39726959900411316772e-4_f64) * t38647;
    let t42702 = F::cast_from(0.15965655602485078085e0_f64) * t38675;
    let t42712 = F::cast_from(0.35754263910370185096e-3_f64) * t38704;
    let t42714 = F::cast_from(0.47672351880493580128e-3_f64) * t38710;
    let t42715 = F::cast_from(0.11918087970123395032e-3_f64) * t38712;
    (t42621, t42665, t42666, t42685, t42693, t42696, t42697, t42698, t42702, t42712, t42714, t42715)
}
