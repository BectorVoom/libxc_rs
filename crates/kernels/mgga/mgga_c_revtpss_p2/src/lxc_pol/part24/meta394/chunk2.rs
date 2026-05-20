//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1310/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1310<F: Float>(t234: F, t251: F, t268: F, t39644: F, t8779: F, t39497: F, t874: F, t875: F, t10529: F, t2453: F, t253: F, t39552: F) -> (F, F, F, F) {
    let t39649 = F::cast_from(0.11638313500518478545e-4_f64) * t39644 * t234 * t251 * t8779 * t268;
    let t39652 = F::cast_from(0.10118827226026589797e0_f64) * t874 * t875 * t39497;
    let t39680 = t2453 * t10529;
    let t39697 = F::cast_from(0.88356352675825229576e-3_f64) * t39552 * t253;
    (t39649, t39652, t39680, t39697)
}
