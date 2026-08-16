//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1400/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1400<F: Float>(t47371: F, t786: F, t1432: F, t1433: F, t39497: F, t10111: F, t1428: F, t588: F, t10022: F, t2453: F, t268: F, t39644: F, t546: F, t555: F, t8779: F) -> (F, F, F, F, F) {
    let t47372 = t786 * t47371;
    let t47395 = F::cast_from(0.10118827226026589797e0_f64) * t1432 * t1433 * t39497;
    let t47417 = F::cast_from(0.15709759505761725819e-2_f64) * t10111 * t1428 * t588;
    let t47429 = t2453 * t10022;
    let t47442 = F::cast_from(0.11638313500518478545e-4_f64) * t39644 * t546 * t555 * t8779 * t268;
    (t47372, t47395, t47417, t47429, t47442)
}
