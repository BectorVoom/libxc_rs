//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2100/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2100<F: Float>(t24574: F, t27484: F, t24826: F, t27540: F, t210: F, t24848: F, t27505: F, t27466: F, t27455: F, t27474: F, t27492: F, t85853: F) -> (F, F, F, F, F, F, F) {
    let t95048 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27484;
    let t95069 = F::cast_from(0.54831135561607547884e-2_f64) * t24826 * t27540;
    let t95092 = t27505 * t210 * t24848;
    let t95098 = F::cast_from(0.18277045187202515961e-2_f64) * t24574 * t27466;
    let t95114 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27455;
    let t95125 = F::cast_from(0.18277045187202515961e-2_f64) * t24574 * t27474;
    let t95134 = F::cast_from(0.10966227112321509577e-1_f64) * t85853 * t27492;
    (t95048, t95069, t95092, t95098, t95114, t95125, t95134)
}
