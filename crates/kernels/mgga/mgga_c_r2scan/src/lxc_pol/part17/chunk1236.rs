//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1236/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1236<F: Float>(t38033: F, t41649: F, t41651: F, t43459: F, t43462: F, t43465: F, t43468: F, t43471: F, t43474: F, t43477: F, t43480: F, t43483: F) -> F {
    let t44407 = F::new(0.17465477326173296718e-1) * t43459 + F::new(0.26198215989259945076e-1) * t43462 - F::new(0.87327386630866483588e-2) * t43465 + F::new(0.26198215989259945076e-1) * t43468 + F::new(0.1047928639570397803e0) * t43471 + t41649 + t41651 + F::new(0.86682217400542685632e-1) * t43474 - F::new(0.87327386630866483588e-2) * t43477 + F::new(0.31147743054556651237e-1) * t38033 - F::new(0.87327386630866483588e-2) * t43480 - F::new(0.43663693315433241794e-2) * t43483;
    t44407
}
