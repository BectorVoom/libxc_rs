//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 764/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk764<F: Float>(t1614: F, t2084: F, t2139: F, t27: F, t34884: F, t9123: F, t4601: F, t9008: F, t40906: F, t8640: F, t2038: F, t39116: F, t7756: F, t7933: F, t2049: F, t35688: F, t7760: F) -> (F, F, F, F, F, F) {
    let t42132 = t2139 * t27 * t2084 * t1614;
    let t42144 = t34884 * t9123;
    let t42151 = t4601 * t9008;
    let t42166 = t8640 * t40906;
    let t42170 = t7933 * t2038 * t39116 * t7756;
    let t42174 = t35688 * t2049 * t39116 * t7760;
    (t42132, t42144, t42151, t42166, t42170, t42174)
}
