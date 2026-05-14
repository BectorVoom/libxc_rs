//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 584/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk584<F: Float>(t1899: F, t6702: F, t1873: F, t1869: F, t1757: F, t2441: F) -> (F, F, F, F) {
    let t6703 = t1899 * t6702;
    let t6704 = t1873 * t6703;
    let t6705 = t1869 * t6704;
    let t6707 = t2441 * t1757;
    (t6703, t6704, t6705, t6707)
}
