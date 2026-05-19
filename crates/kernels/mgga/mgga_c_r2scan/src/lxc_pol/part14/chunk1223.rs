//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1223/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1223<F: Float>(t39939: F, t39941: F, t37985: F, t37988: F, t37998: F, t38003: F, t38597: F, t39937: F, t39945: F, t39947: F, t39951: F, t39954: F) -> F {
    let t41623 = F::cast_from(0.21951497276451705328e-1_f64) * t39939;
    let t41624 = F::cast_from(0.46230515946956099004e0_f64) * t39941;
    let t41633 = F::cast_from(0.1047928639570397803e0_f64) * t39937 - t41623 + t41624 + F::cast_from(0.23804984598836975486e0_f64) * t37985 - F::cast_from(0.27738309568173659402e1_f64) * t37988 - F::cast_from(0.86682217400542685632e-1_f64) * t39945 + F::cast_from(0.5200933044032561138e0_f64) * t39947 + F::cast_from(0.19514881078765566037e-1_f64) * t37998 - t38597 + F::cast_from(0.65049603595885220124e-3_f64) * t38003 + F::cast_from(0.86682217400542685632e-1_f64) * t39951 + F::cast_from(0.26198215989259945076e-1_f64) * t39954;
    t41633
}
