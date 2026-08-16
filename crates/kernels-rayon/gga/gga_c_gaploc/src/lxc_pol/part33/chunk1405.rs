//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1405/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1405(t11982: f64, t1531: f64, t31357: f64, t31360: f64, t31382: f64, t31386: f64, t31393: f64, t35172: f64, t35174: f64, t35178: f64, t35183: f64, t35185: f64, t35188: f64, t35192: f64, t35199: f64, t38731: f64, t4614: f64, t597: f64, t6963: f64, t6964: f64, t7025: f64) -> f64 {
    let t38850 = 0.30674340763136599741e2_f64 * t597 * t4614 * t11982 - 0.15337170381568299871e1_f64 * t31357 - 0.15337170381568299871e1_f64 * t31360 + 0.21450293971110256002e1_f64 * t7025 * t1531 * t38731 - 0.14300195980740170668e1_f64 * t6963 * t6964 * t38731 + t35172 - t35174 - t35178 - t31382 + t31386 + 0.15337170381568299871e1_f64 * t31393 + t35183 - t35185 + t35188 - t35192 - t35199;
    t38850
}
