//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2217/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2217<F: Float>(t2247: F, t30681: F, t38: F, t108733: F, t26749: F, t26755: F, t28112: F, t28116: F, t28119: F, t28133: F, t28141: F, t29372: F, t29388: F, t29544: F, t30683: F, t6960: F, t6963: F, t7566: F, t7709: F, t8144: F, t8147: F) -> F {
    let t111516 = t2247 * t38 * t30681;
    let t111521 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t26749 * t29544 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t26755 * t29544 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7566 * t108733 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28112 * t8147 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28116 * t8147 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28119 * t8147 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7709 * t29372 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28141 * t8144 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t29388 * t28133 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28141 * t8147 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t111516 * t6960 + t6963 * t30683 / F::cast_from(3.0_f64);
    t111521
}
