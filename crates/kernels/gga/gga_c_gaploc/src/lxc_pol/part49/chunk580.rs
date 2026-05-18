//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 580/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk580<F: Float>(t787: F, t9816: F, t2021: F, t2672: F, t7372: F, t7634: F, t2558: F, t9286: F) -> (F, F, F, F) {
    let t9817 = t787 * t9816;
    let t9820 = t2021 * t2672;
    let t9822 = F::new(0.29792074959875355558e-1) * t9820 * t7372;
    let t9823 = t787 * t7634;
    let t9824 = t9286 * t2558;
    (t9817, t9822, t9823, t9824)
}
