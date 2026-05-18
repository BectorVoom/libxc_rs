//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 512/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk512<F: Float>(t7372: F, t9820: F, t7634: F, t787: F, t2558: F, t9286: F) -> (F, F, F) {
    let t9822 = F::new(0.29792074959875355558e-1) * t9820 * t7372;
    let t9823 = t787 * t7634;
    let t9824 = t9286 * t2558;
    (t9822, t9823, t9824)
}
