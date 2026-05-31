//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 774/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk774<F: Float>(t2253: F, t3642: F, t11034: F, t3613: F, t1736: F, t179: F, t11008: F, t12099: F, t12102: F, t12104: F, t12108: F, t12113: F, t12119: F, t12123: F, t12128: F, t2265: F, t631: F, t8641: F, t8643: F, t8645: F, t8647: F, t8676: F, t8678: F, t8714: F, t8718: F, t8719: F) -> F {
    let t12132 = F::cast_from(2.0_f64) * t2253 * t3642;
    let t12134 = t3613 * t11034;
    let t12137 = t1736 * t179;
    let t12138 = t12137 * t11008;
    let t12141 = F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t8641 - t8643 / F::cast_from(9.0_f64) - t8645 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2265 * t12099 + t2265 * t12102 - t2265 * t12104 / F::cast_from(3.0_f64) + t2265 * t12108 + t8647 - t8714 / F::cast_from(3.0_f64) + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t8719 + F::cast_from(2.0_f64) * t2265 * t12113 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2265 * t12119 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2265 * t12123 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t8676 + t8718 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t631 * t12128 + t12132 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t8678 + t2265 * t12134 / F::cast_from(18.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t2265 * t12138;
    t12141
}
