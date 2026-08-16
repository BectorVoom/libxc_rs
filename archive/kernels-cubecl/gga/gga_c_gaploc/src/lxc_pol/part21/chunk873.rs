//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 873/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk873<F: Float>(t107: F, t2931: F, t2021: F, t1858: F, t3038: F, t787: F, t2610: F, t8669: F) -> (F, F, F, F) {
    let t8748 = t2931 * t107;
    let t8749 = t2021 * t8748;
    let t8752 = t1858 * t3038;
    let t8753 = t787 * t8752;
    let t8756 = t2610 * t8669;
    (t8749, t8752, t8753, t8756)
}
