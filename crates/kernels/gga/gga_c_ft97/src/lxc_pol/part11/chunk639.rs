//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 639/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk639<F: Float>(t184: F, t9470: F, t1580: F, t185: F, t21: F, t2236: F, t2240: F, t2301: F, t2306: F, t2309: F, t363: F, t5: F, t620: F, t623: F, t650: F, t7745: F, t8614: F, t8724: F, t8732: F, t8739: F, t8744: F, t8751: F, t8754: F) -> (F, F) {
    let t9471 = t9470 * t184;
    let t9478 = 3.0 / 4.0 * t8614 * t650 + t623 * t8724 / 4.0 + 3.0 / 4.0 * t5 * t2236 * t363 + t623 * t8732 / 4.0 + 3.0 / 4.0 * t5 * t620 * t1580 + 3.0 / 4.0 * t623 * t8739 + 3.0 / 4.0 * t2240 * t2306 + 3.0 / 4.0 * t623 * t8744 + 3.0 / 4.0 * t2240 * t2301 + 3.0 / 2.0 * t2240 * t2309 + 3.0 / 4.0 * t623 * t8751 + 3.0 / 4.0 * t623 * t8754 + t5 * t9471 * t21 / 4.0 + t5 * t185 * t7745 / 4.0;
    (t9471, t9478)
}
