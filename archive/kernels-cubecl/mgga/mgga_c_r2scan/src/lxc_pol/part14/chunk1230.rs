//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1230/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1230<F: Float>(t40155: F, t40157: F, t40162: F, t38096: F, t38099: F, t38111: F, t38114: F, t40145: F, t40149: F, t40151: F, t40153: F, t40165: F) -> F {
    let t41721 = F::cast_from(0.93149212406257582492e-1_f64) * t40155;
    let t41722 = F::cast_from(0.46230515946956099004e0_f64) * t40157;
    let t41725 = F::cast_from(0.27944763721877274748e0_f64) * t40162;
    let t41727 = F::cast_from(0.43663693315433241794e-2_f64) * t40145 - F::cast_from(0.93149212406257582492e-1_f64) * t38096 - F::cast_from(0.27944763721877274748e0_f64) * t38099 - F::cast_from(0.17465477326173296718e-1_f64) * t40149 - F::cast_from(0.26198215989259945076e-1_f64) * t40151 + F::cast_from(0.87327386630866483588e-2_f64) * t40153 - t41721 + t41722 - F::cast_from(0.25610080155860322884e0_f64) * t38111 - F::cast_from(0.46230515946956099004e0_f64) * t38114 - t41725 + F::cast_from(0.43663693315433241794e-2_f64) * t40165;
    t41727
}
