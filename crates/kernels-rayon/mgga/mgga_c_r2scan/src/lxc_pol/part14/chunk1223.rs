//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1223/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1223(t39939: f64, t39941: f64, t37985: f64, t37988: f64, t37998: f64, t38003: f64, t38597: f64, t39937: f64, t39945: f64, t39947: f64, t39951: f64, t39954: f64) -> f64 {
    let t41623 = 0.21951497276451705328e-1_f64 * t39939;
    let t41624 = 0.46230515946956099004e0_f64 * t39941;
    let t41633 = 0.1047928639570397803e0_f64 * t39937 - t41623 + t41624 + 0.23804984598836975486e0_f64 * t37985 - 0.27738309568173659402e1_f64 * t37988 - 0.86682217400542685632e-1_f64 * t39945 + 0.5200933044032561138e0_f64 * t39947 + 0.19514881078765566037e-1_f64 * t37998 - t38597 + 0.65049603595885220124e-3_f64 * t38003 + 0.86682217400542685632e-1_f64 * t39951 + 0.26198215989259945076e-1_f64 * t39954;
    t41633
}
