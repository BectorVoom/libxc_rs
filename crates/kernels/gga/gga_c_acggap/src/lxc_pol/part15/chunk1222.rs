//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1222/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1222<F: Float>(t32507: F, t32509: F, t34431: F, t34432: F, t34433: F, t37087: F, t37090: F, t39299: F, t39302: F, t39305: F, t39308: F, t39311: F, t39314: F, t39318: F, t39320: F, t39322: F, t39324: F, t39326: F) -> F {
    let t41582 = -t37087 - F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t39299 - t39302 / F::cast_from(16.0_f64) + t39305 / F::cast_from(8.0_f64) + t39308 / F::cast_from(32.0_f64) + t39311 / F::cast_from(32.0_f64) - t39314 / F::cast_from(32.0_f64) - t32507 - t37090 - t34431 + t32509 + t34432 - F::cast_from(77.0_f64) / F::cast_from(432.0_f64) * t34433 + t39318 / F::cast_from(24.0_f64) - F::cast_from(0.16809375e0_f64) * t39320 + F::cast_from(0.16809375e0_f64) * t39322 + F::cast_from(0.3361875e0_f64) * t39324 - F::cast_from(11.0_f64) / F::cast_from(96.0_f64) * t39326;
    t41582
}
