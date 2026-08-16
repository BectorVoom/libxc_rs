//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2409/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2409(t47707: f64, t48096: f64, t41831: f64, t41833: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47722: f64, t47724: f64, t47728: f64) -> f64 {
    let t49306 = 0.45908888888888888888e0_f64 * t47707;
    let t49317 = 0.34731666666666666667e0_f64 * t48096;
    let t49318 = -t49306 + 0.68863333333333333333e0_f64 * t47709 + 0.34431666666666666666e0_f64 * t47711 + 0.57386111111111111111e0_f64 * t47713 - 0.20659e1_f64 * t47715 - 0.103295e1_f64 * t47717 - 0.17215833333333333333e1_f64 * t47722 - 0.20658999999999999999e1_f64 * t47724 - 0.123954e2_f64 * t47728 + 0.69463333333333333333e0_f64 * t41831 + 0.41678000000000000001e0_f64 * t41833 - t49317;
    t49318
}
