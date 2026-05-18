//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1128/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1128<F: Float>(t27842: F, t4074: F, t4077: F, t18091: F, t27847: F, t18089: F, t18096: F, t27846: F, t4066: F, t92: F, t4082: F, t4085: F) -> (F, F, F, F) {
    let t29901 = t27842 * t4074 * t4077;
    let t29903 = t27847 * t18091;
    let t29908 = t18096 * t4066 * t27846 * t18089 * t92;
    let t29911 = t4082 * t27842 * t4085;
    (t29901, t29903, t29908, t29911)
}
