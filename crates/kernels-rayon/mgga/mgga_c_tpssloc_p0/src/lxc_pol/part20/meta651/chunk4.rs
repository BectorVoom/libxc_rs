//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2398/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2398(t41831: f64, t41833: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47722: f64, t47724: f64, t47728: f64, t49139: f64) -> f64 {
    let t49140 = -0.26837777777777777778e0_f64 * t47707 + 0.40256666666666666667e0_f64 * t47709 + 0.20128333333333333333e0_f64 * t47711 + 0.33547222222222222222e0_f64 * t47713 - 0.12077e1_f64 * t47715 - 0.60385e0_f64 * t47717 - 0.10064166666666666666e1_f64 * t47722 - 0.12077e1_f64 * t47724 - 0.72462e1_f64 * t47728 + 0.55190000000000000001e0_f64 * t41831 + 0.33114e0_f64 * t41833 - t49139;
    t49140
}
