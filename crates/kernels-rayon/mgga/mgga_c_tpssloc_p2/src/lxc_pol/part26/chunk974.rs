//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 974/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk974(t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11150: f64, t11156: f64, t11165: f64, t11174: f64, t11230: f64, t11233: f64, t11245: f64, t11259: f64, t11261: f64, t11266: f64) -> f64 {
    let t11398 = -0.82785e-1_f64 * t11230 + 0.49671e0_f64 * t11233 + 0.40256666666666666668e0_f64 * t11137 + 0.20128333333333333333e0_f64 * t11139 - 0.60385000000000000001e0_f64 * t11141 - 0.30192500000000000001e0_f64 * t11143 + 0.33547222222222222222e0_f64 * t11150 - 0.12077e1_f64 * t11156 + 0.181155e1_f64 * t11165 + 0.301925e0_f64 * t11174 - 0.412621875e-1_f64 * t11245 + 0.258925e1_f64 * t11259 + 0.16504875e0_f64 * t11261 + 0.19419375e1_f64 * t11266;
    t11398
}
