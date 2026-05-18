//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1222/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1222<F: Float>(t39899: F, t39903: F, t39906: F, t39911: F, t39920: F, t37983: F, t39908: F, t39914: F, t39916: F, t39924: F, t39927: F, t39930: F) -> F {
    let t41607 = F::new(0.93149212406257582492e-1) * t39899;
    let t41608 = F::new(0.27944763721877274748e0) * t39903;
    let t41609 = F::new(0.13506635798907349462e1) * t39906;
    let t41611 = F::new(0.46230515946956099004e0) * t39911;
    let t41615 = F::new(0.28565981518604370584e-1) * t39920;
    let t41619 = -t41607 - t41608 + t41609 - F::new(0.65854491829355115984e-1) * t39908 - t41611 - F::new(0.10401866088065122276e1) * t39914 - F::new(0.10975748638225852664e0) * t39916 + F::new(0.39029762157531132074e-1) * t37983 + t41615 + F::new(0.17465477326173296718e-1) * t39924 + F::new(0.26198215989259945076e-1) * t39927 + F::new(0.26198215989259945076e-1) * t39930;
    t41619
}
