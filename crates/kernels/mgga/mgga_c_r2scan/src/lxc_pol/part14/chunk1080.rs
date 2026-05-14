//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1080/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1080<F: Float>(t39920: F, t37983: F, t39908: F, t39914: F, t39916: F, t39924: F, t39927: F, t39930: F, t41607: F, t41608: F, t41609: F, t41611: F, t39939: F, t39941: F, t37985: F, t37988: F, t37998: F, t38003: F, t38597: F, t39937: F, t39945: F, t39947: F, t39951: F, t39954: F) -> (F, F) {
    let t41615 = 0.28565981518604370584e-1 * t39920;
    let t41619 = -t41607 - t41608 + t41609 - 0.65854491829355115984e-1 * t39908 - t41611 - 0.10401866088065122276e1 * t39914 - 0.10975748638225852664e0 * t39916 + 0.39029762157531132074e-1 * t37983 + t41615 + 0.17465477326173296718e-1 * t39924 + 0.26198215989259945076e-1 * t39927 + 0.26198215989259945076e-1 * t39930;
    let t41623 = 0.21951497276451705328e-1 * t39939;
    let t41624 = 0.46230515946956099004e0 * t39941;
    let t41633 = 0.1047928639570397803e0 * t39937 - t41623 + t41624 + 0.23804984598836975486e0 * t37985 - 0.27738309568173659402e1 * t37988 - 0.86682217400542685632e-1 * t39945 + 0.5200933044032561138e0 * t39947 + 0.19514881078765566037e-1 * t37998 - t38597 + 0.65049603595885220124e-3 * t38003 + 0.86682217400542685632e-1 * t39951 + 0.26198215989259945076e-1 * t39954;
    (t41619, t41633)
}
