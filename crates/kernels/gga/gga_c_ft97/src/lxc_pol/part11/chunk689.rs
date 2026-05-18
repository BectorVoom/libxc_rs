//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 689/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk689<F: Float>(t2379: F, t9542: F, t3724: F, t694: F, t709: F, t200: F, t9525: F, t191: F, t2999: F) -> (F, F, F, F) {
    let t9543 = t2379 * t9542;
    let t9545 = t3724 * t694 * t709;
    let t9548 = t9525 * t200;
    let t9555 = t2999 * t191;
    (t9543, t9545, t9548, t9555)
}
