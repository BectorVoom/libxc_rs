//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 552/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk552<F: Float>(t2679: F, t948: F, t9796: F, t7809: F, t822: F) -> (F, F) {
    let t9797 = t948 * t2679;
    let t9798 = t9796 * t9797;
    let t9799 = F::cast_from(0.76685851907841499352e0_f64) * t9798;
    let t9800 = t822 * t7809;
    (t9799, t9800)
}
