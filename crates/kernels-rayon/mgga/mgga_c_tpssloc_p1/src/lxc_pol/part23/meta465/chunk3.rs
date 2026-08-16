//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1363/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1363(t77058: f64, t77071: f64, t901: f64, t5698: f64, t41935: f64, t59657: f64, t60168: f64, t60173: f64, t60204: f64, t68502: f64, t68504: f64, t68506: f64, t76877: f64, t76880: f64, t76887: f64, t76890: f64, t77042: f64) -> (f64, f64, f64, f64, f64) {
    let t77072 = t77058 + t77071;
    let t77073 = t901 * t77072;
    let t77075 = t5698 * t5698;
    let t77076 = t41935 * t77075;
    let t77082 = 0.21908444444444444444e0_f64 * t68502 + 0.13145066666666666666e1_f64 * t68504 - 0.43816888888888888888e0_f64 * t68506 + 0.46074375e0_f64 * t77042 + 0.10954222222222222222e1_f64 * t60168 - 0.54771111111111111111e0_f64 * t60173 - 0.5314962962962962963e0_f64 * t59657 + 0.98587999999999999999e0_f64 * t76880 + 0.3071625e0_f64 * t77073 - 0.3560484375e1_f64 * t77076 - 0.18257037037037037037e0_f64 * t60204 - 0.82156666666666666668e-1_f64 * t76877 - 0.85199506172839506175e-1_f64 * t76887 - 0.82156666666666666667e-1_f64 * t76890;
    (t77072, t77073, t77075, t77076, t77082)
}
