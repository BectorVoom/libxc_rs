//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1069/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1069<F: Float>(t38033: F, t39996: F, t40001: F, t43459: F, t43462: F, t43465: F, t43468: F, t43471: F, t43474: F, t43477: F, t43480: F, t43483: F, t11693: F, t8198: F, t10856: F, t9319: F) -> (F, F, F) {
    let t43485 = 0.87327386630866483584e-2 * t43459 + 0.13099107994629972538e-1 * t43462 - 0.43663693315433241792e-2 * t43465 + 0.13099107994629972538e-1 * t43468 + 0.52396431978519890152e-1 * t43471 + t39996 + 0.13972381860938637374e0 * t40001 + 0.43341108700271342816e-1 * t43474 - 0.43663693315433241792e-2 * t43477 + 0.15573871527278325618e-1 * t38033 - 0.43663693315433241792e-2 * t43480 - 0.21831846657716620896e-2 * t43483;
    let t43488 = t8198 * t11693;
    let t43490 = t10856 * t9319;
    (t43485, t43488, t43490)
}
