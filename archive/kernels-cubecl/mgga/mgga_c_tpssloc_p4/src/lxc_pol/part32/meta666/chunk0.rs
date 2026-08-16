//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2098/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2098<F: Float>(t1193: F, t27506: F, t7378: F, t11153: F, t491: F, t24826: F, t27537: F, t27526: F, t86094: F, t24660: F, t24850: F, t24667: F) -> (F, F, F, F, F, F, F) {
    let t94909 = t27506 * t1193;
    let t94911 = F::cast_from(0.14621636149762012769e-1_f64) * t94909 * t7378;
    let t94920 = t491 * t11153;
    let t94941 = F::cast_from(0.54831135561607547884e-2_f64) * t24826 * t27537;
    let t94947 = F::cast_from(0.18277045187202515961e-2_f64) * t86094 * t27526;
    let t94948 = t24660 * t24850;
    let t94954 = t24667 * t24850;
    (t94909, t94911, t94920, t94941, t94947, t94948, t94954)
}
