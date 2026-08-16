//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 972/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk972(t1100: f64, t11258: f64, t1107: f64, t410: f64, t417: f64, t11244: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11150: f64, t11156: f64, t11165: f64, t11174: f64, t11230: f64, t11233: f64, t11245: f64) -> (f64, f64, f64, f64, f64) {
    let t11259 = t1100 * t11258;
    let t11261 = t1107 * t11258;
    let t11265 = 1.0_f64 / t410 / t417 / 4.0_f64;
    let t11266 = t11265 * t11244;
    let t11268 = -0.82156666666666666668e-1_f64 * t11230 + 0.49293999999999999999e0_f64 * t11233 + 0.39862222222222222223e0_f64 * t11137 + 0.19931111111111111111e0_f64 * t11139 - 0.59793333333333333333e0_f64 * t11141 - 0.29896666666666666667e0_f64 * t11143 + 0.33218518518518518518e0_f64 * t11150 - 0.11958666666666666667e1_f64 * t11156 + 0.17938e1_f64 * t11165 + 0.29896666666666666667e0_f64 * t11174 - 0.76790625e-1_f64 * t11245 + 0.1898925e1_f64 * t11259 + 0.3071625e0_f64 * t11261 + 0.142419375e1_f64 * t11266;
    (t11259, t11261, t11265, t11266, t11268)
}
