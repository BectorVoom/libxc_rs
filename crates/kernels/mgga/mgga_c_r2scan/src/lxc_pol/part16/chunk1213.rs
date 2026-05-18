//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1213/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1213<F: Float>(t11717: F, t26278: F, t10760: F, t29700: F, t6085: F, t38033: F, t39996: F, t40001: F, t43459: F, t43462: F, t43465: F, t43468: F, t43471: F, t43474: F, t43477: F) -> F {
    let t43480 = t26278 * t11717;
    let t43483 = t6085 * t10760 * t29700;
    let t43485 = F::new(0.87327386630866483584e-2) * t43459 + F::new(0.13099107994629972538e-1) * t43462 - F::new(0.43663693315433241792e-2) * t43465 + F::new(0.13099107994629972538e-1) * t43468 + F::new(0.52396431978519890152e-1) * t43471 + t39996 + F::new(0.13972381860938637374e0) * t40001 + F::new(0.43341108700271342816e-1) * t43474 - F::new(0.43663693315433241792e-2) * t43477 + F::new(0.15573871527278325618e-1) * t38033 - F::new(0.43663693315433241792e-2) * t43480 - F::new(0.21831846657716620896e-2) * t43483;
    t43485
}
