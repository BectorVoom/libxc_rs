//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1264/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1264(t100656: f64, t100660: f64, t100666: f64, t100669: f64, t100672: f64, t2192: f64, t2197: f64, t27042: f64, t29094: f64, t70078: f64, t96121: f64, t97265: f64, t97273: f64, t97281: f64) -> f64 {
    let t100674 = 0.37101880208333333333e-3_f64 * t27042 * t29094 - 0.46377350260416666667e-4_f64 * t100656 + t97265 - 0.51588271604938271603e-3_f64 * t96121 - t97273 - 0.92858888888888888885e-2_f64 * t100660 - t97281 - 0.34752604166666666667e-3_f64 * t70078 * t2192 * t2197 + 0.61905925925925925925e-2_f64 * t100666 + 0.46429444444444444444e-2_f64 * t100669 + 0.11607361111111111111e-2_f64 * t100672;
    t100674
}
