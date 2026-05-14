//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 599/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk599<F: Float>(t50: F, t34: F, t893: F, t238: F, t821: F, t1369: F, t1372: F, t39: F, t4011: F, t52: F, t830: F, t833: F, t4010: F, t59: F, zeta_threshold: F) -> (F, F) {
    let t51 = t50 <= zeta_threshold;
    let t4014 = t893 * t34;
    let t4015 = t821 * t238;
    let t4025 = piecewise3(t51, 0.0, -8.0 / 27.0 * t4011 * t830 - 16.0 / 9.0 * t4014 * t4015 + 4.0 / 9.0 * t1369 * t833 - 8.0 / 3.0 * t52 * t821 + 8.0 * t1372 * t39);
    let t4027 = (t4010 + t4025) * t59;
    (t4015, t4027)
}
