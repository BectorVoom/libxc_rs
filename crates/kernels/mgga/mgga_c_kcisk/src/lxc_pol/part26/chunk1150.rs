//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1150/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1150<F: Float>(t33643: F, t9492: F, t32278: F, t6328: F, t19861: F, t500: F, t488: F, t6309: F, t32277: F, t3784: F) -> (F, F, F, F, F) {
    let t33644 = t33643 * t9492;
    let t33646 = t32278 * t6328;
    let t33648 = t19861 * t500;
    let t33650 = t6309 * t488;
    let t33652 = t3784 * t32277;
    (t33644, t33646, t33648, t33650, t33652)
}
