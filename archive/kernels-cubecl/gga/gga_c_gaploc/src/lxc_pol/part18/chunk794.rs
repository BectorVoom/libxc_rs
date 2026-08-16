//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 794/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk794<F: Float>(t107: F, t2536: F, t2021: F, t1858: F, t2652: F, t787: F, t4820: F, t7069: F, t4598: F, t965: F, t4585: F, t948: F) -> (F, F, F, F, F, F) {
    let t7526 = t2536 * t107;
    let t7527 = t2021 * t7526;
    let t7530 = t1858 * t2652;
    let t7531 = t787 * t7530;
    let t7534 = t4820 * t7069;
    let t7539 = t4598 * t965;
    let t7542 = t4585 * t948;
    (t7527, t7530, t7531, t7534, t7539, t7542)
}
