//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 260/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk260<F: Float>(t270: F, t283: F, t61: F, t248: F, t884: F, t1000: F, t1005: F, t1020: F, t1025: F, t1032: F, t1038: F, t1041: F, t350: F, t378: F, t964: F, t973: F, t997: F) -> (F, F, F, F) {
    let t1043 = F::cast_from(1.0_f64) / t283 / t270;
    let t1044 = t61 * t1043;
    let t1046 = t248 * t1044 * t884;
    let t1049 = -t964 * t350 / F::cast_from(36.0_f64) + t997 + t973 * t1000 / F::cast_from(288.0_f64) + t1005 * t378 / F::cast_from(3072.0_f64) + t1020 * t1025 / F::cast_from(3072.0_f64) - t1032 * t378 / F::cast_from(576.0_f64) + t1038 + t1041 * t1046 / F::cast_from(4608.0_f64);
    (t1043, t1044, t1046, t1049)
}
