//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 857/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk857<F: Float>(t1614: F, t53: F, t51: F, t5596: F, t1608: F, t397: F, t1712: F, t373: F, t384: F, t401: F, t7900: F, t1632: F, t1685: F) -> (F, F, F, F, F, F) {
    let t37545 = t1614 * t53;
    let t37550 = t5596 * t51;
    let t37551 = t1608 * t37550;
    let t37552 = t37545 * t397;
    let t37554 = t1712 * t373 * t384;
    let t37558 = t7900 * t401;
    let t37570 = t1632 * t1685;
    (t37545, t37551, t37552, t37554, t37558, t37570)
}
