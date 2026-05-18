//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 809/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk809<F: Float>(t12629: F, t731: F, t12604: F, t12608: F, t12612: F, t12623: F, t2549: F, t948: F, t9796: F, t9829: F, t1967: F, t28236: F, t7810: F, t883: F) -> (F, F, F, F, F, F, F, F) {
    let t40877 = t731 * t12629;
    let t40890 = t731 * t12604;
    let t40896 = t731 * t12608;
    let t40898 = t731 * t12612;
    let t40900 = t731 * t12623;
    let t40902 = t2549 * t12629;
    let t40942 = t9796 * t948 * t9829;
    let t40946 = t7810 * t1967 * t883 * t28236;
    (t40877, t40890, t40896, t40898, t40900, t40902, t40942, t40946)
}
