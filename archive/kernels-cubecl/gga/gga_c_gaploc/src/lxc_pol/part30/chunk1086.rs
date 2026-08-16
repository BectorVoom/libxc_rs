//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1086/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1086<F: Float>(t15667: F, t27847: F, t15665: F, t15672: F, t27846: F, t4066: F, t92: F, t27842: F, t5345: F, t5348: F, t1695: F, t3220: F) -> (F, F, F, F) {
    let t27848 = t27847 * t15667;
    let t27853 = t15672 * t4066 * t27846 * t15665 * t92;
    let t27856 = t5345 * t27842 * t5348;
    let t27858 = t3220 * t1695;
    (t27848, t27853, t27856, t27858)
}
