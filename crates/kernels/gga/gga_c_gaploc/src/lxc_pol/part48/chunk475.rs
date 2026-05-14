//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 475/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk475<F: Float>(t2624: F, t2679: F, t9800: F, t5638: F, t6574: F, t822: F) -> (F, F, F) {
    let t9801 = t2624 * t2679;
    let t9803 = 0.19171462976960374838e1 * t9800 * t9801;
    let t9804 = t5638 * t6574;
    let t9805 = t822 * t9804;
    (t9803, t9804, t9805)
}
