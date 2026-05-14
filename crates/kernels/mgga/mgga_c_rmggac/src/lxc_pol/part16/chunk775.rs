//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 775/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk775<F: Float>(t38639: F, t38643: F, t38645: F, t38647: F, t38675: F, t38704: F, t38710: F, t38712: F, t38775: F, t38818: F, t38837: F, t38853: F, t38857: F, t38860: F, t38863: F, t38869: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42693 = 0.39726959900411316772e-4 * t38639;
    let t42696 = 0.11918087970123395032e-3 * t38643;
    let t42697 = 0.11918087970123395032e-3 * t38645;
    let t42698 = 0.39726959900411316772e-4 * t38647;
    let t42702 = 0.15965655602485078085e0 * t38675;
    let t42712 = 0.35754263910370185096e-3 * t38704;
    let t42714 = 0.47672351880493580128e-3 * t38710;
    let t42715 = 0.11918087970123395032e-3 * t38712;
    let t42740 = 0.36366215538993788974e-1 * t38775;
    let t42749 = 0.1440846329149835838e-2 * t38818;
    let t42755 = 0.1440846329149835838e-2 * t38837;
    let t42759 = 0.20496175532535769482e-3 * t38853;
    let t42760 = 0.1440846329149835838e-2 * t38857;
    let t42761 = 0.1440846329149835838e-2 * t38860;
    let t42762 = 0.1440846329149835838e-2 * t38863;
    let t42764 = 0.20496175532535769482e-3 * t38869;
    (t42693, t42696, t42697, t42698, t42702, t42712, t42714, t42715, t42740, t42749, t42755, t42759, t42760, t42761, t42762, t42764)
}
