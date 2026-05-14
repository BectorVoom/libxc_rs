//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 835/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk835<F: Float>(t236: F, t498: F, t7230: F, t9210: F, t9988: F, t321: F, t7248: F, t1810: F, t7754: F, t2010: F, t7756: F, t7349: F, t7760: F, t9719: F, t1587: F, t2347: F) -> (F, F, F, F, F) {
    let t45696 = t7230 * t9210 * t236 * t9988 * t498;
    let t45701 = t7230 * t7248 * t236 * t9988 * t321;
    let t45707 = t7754 * t1810;
    let t45709 = t2010 * t45707 * t7756;
    let t45716 = t7349 * t9719 * t7760;
    let t45720 = t2347 * t1587;
    (t45696, t45701, t45709, t45716, t45720)
}
