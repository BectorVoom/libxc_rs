//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2323/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2323(t25927: f64, t98030: f64, t23788: f64, t98011: f64, t1081: f64, t5664: f64, t100638: f64, t100641: f64, t100644: f64, t100646: f64, t100651: f64, t100656: f64, t1649: f64, t1877: f64, t22959: f64, t23295: f64, t25013: f64, t25354: f64, t25372: f64, t25892: f64, t25921: f64, t28771: f64, t6670: f64, t81483: f64, t86736: f64, t97972: f64, t99064: f64) -> f64 {
    let t100659 = t25927 * t98030;
    let t100664 = t23788 * t98011;
    let t100669 = t1081 * t5664;
    let t100674 = -3.0_f64 * t25013 * t100638 + 3.0_f64 * t25013 * t100641 + t25372 * t100644 - t1877 * t6670 * t100646 / 2.0_f64 + 6.0_f64 * t25013 * t100651 - 3.0_f64 * t81483 * t28771 - 3.0_f64 * t22959 * t100656 + 2.0_f64 * t25372 * t100659 - 3.0_f64 * t86736 * t25921 - 3.0_f64 / 2.0_f64 * t22959 * t100664 + t1877 * t25354 * t1649 + t1877 * t23295 * t100669 - t97972 + 6.0_f64 * t99064 * t25892;
    t100674
}
