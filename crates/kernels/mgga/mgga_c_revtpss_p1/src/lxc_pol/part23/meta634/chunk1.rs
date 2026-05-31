//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2331/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2331<F: Float>(t121: F, t4: F, t131: F, t268: F, t8779: F, t588: F, t9282: F, t239: F, t2456: F) -> (F, F, F, F) {
    let t39484 = t121 * t4;
    let t39490 = F::cast_from(1.0_f64) / t131 / t39484 * t121 * t8779 * t268 / F::cast_from(48.0_f64);
    let t39492 = t9282 * t588;
    let t39494 = t2456 * t239;
    (t39484, t39490, t39492, t39494)
}
