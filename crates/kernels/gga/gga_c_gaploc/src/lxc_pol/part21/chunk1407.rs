//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1407/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1407<F: Float>(t11982: F, t1531: F, t31357: F, t31360: F, t31382: F, t31386: F, t31393: F, t35172: F, t35174: F, t35178: F, t35183: F, t35185: F, t35188: F, t35192: F, t35199: F, t38731: F, t4614: F, t597: F, t6963: F, t6964: F, t7025: F) -> F {
    let t38850 = F::new(0.30674340763136599741e2) * t597 * t4614 * t11982 - F::new(0.15337170381568299871e1) * t31357 - F::new(0.15337170381568299871e1) * t31360 + F::new(0.21450293971110256002e1) * t7025 * t1531 * t38731 - F::new(0.14300195980740170668e1) * t6963 * t6964 * t38731 + t35172 - t35174 - t35178 - t31382 + t31386 + F::new(0.15337170381568299871e1) * t31393 + t35183 - t35185 + t35188 - t35192 - t35199;
    t38850
}
