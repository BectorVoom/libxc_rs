//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 884/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk884<F: Float>(t161: F, t2931: F, t1854: F, t1858: F, t3487: F, t734: F, t7289: F, t8769: F, t8773: F, t1845: F, t5396: F, t8756: F) -> (F, F, F, F, F, F, F) {
    let t8867 = t2931 * t161;
    let t8868 = t8867 * t1854;
    let t8871 = t1858 * t3487;
    let t8872 = t8871 * t734;
    let t8875 = t7289 * t8769;
    let t8878 = t8773 * t161;
    let t8879 = t8878 * t1845;
    let t8882 = t5396 * t8756;
    (t8868, t8871, t8872, t8875, t8878, t8879, t8882)
}
