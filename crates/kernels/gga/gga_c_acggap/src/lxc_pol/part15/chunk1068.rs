//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1068/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1068<F: Float>(t32507: F, t32509: F, t34431: F, t34432: F, t34433: F, t37087: F, t37090: F, t39299: F, t39302: F, t39305: F, t39308: F, t39311: F, t39314: F, t39318: F, t39320: F, t39322: F, t39324: F, t39326: F) -> (F,) {
    let t41582 = -t37087 - 5.0 / 16.0 * t39299 - t39302 / 16.0 + t39305 / 8.0 + t39308 / 32.0 + t39311 / 32.0 - t39314 / 32.0 - t32507 - t37090 - t34431 + t32509 + t34432 - 77.0 / 432.0 * t34433 + t39318 / 24.0 - 0.16809375e0 * t39320 + 0.16809375e0 * t39322 + 0.3361875e0 * t39324 - 11.0 / 96.0 * t39326;
    (t41582,)
}
