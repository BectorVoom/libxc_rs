//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3074/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3074(t43748: f64, t50903: f64, t50905: f64, t50907: f64, t50919: f64, t50921: f64, t50948: f64, t50950: f64, t63327: f64, t63330: f64, t63332: f64, t63334: f64, t63336: f64) -> f64 {
    let t63798 = 0.4274e0_f64 * t63327 - 0.28493333333333333333e0_f64 * t63330 - 0.52765432098765432097e-2_f64 * t63332 + 0.79148148148148148146e-2_f64 * t63334 - 0.11872222222222222222e-1_f64 * t63336 - 0.52765432098765432098e-2_f64 * t43748 - 0.47488888888888888888e-1_f64 * t50903 - 0.23744444444444444444e-1_f64 * t50905 - 0.71233333333333333332e-1_f64 * t50907 - 0.21106172839506172839e-1_f64 * t50919 - 0.13191358024691358025e-1_f64 * t50921 + 0.63318518518518518517e-1_f64 * t50948 + 0.15829629629629629629e-1_f64 * t50950;
    t63798
}
