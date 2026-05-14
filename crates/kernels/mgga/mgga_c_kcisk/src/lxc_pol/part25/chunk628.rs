//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 628/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk628<F: Float>(t1899: F, t6689: F, t1873: F, t1869: F, t2473: F, t4581: F, t1799: F, t2454: F, t719: F) -> (F, F, F, F, F, F) {
    let t6690 = t1899 * t6689;
    let t6691 = t1873 * t6690;
    let t6692 = t1869 * t6691;
    let t6694 = t4581 * t2473;
    let t6695 = t1799 * t6694;
    let t6697 = t2454 * t719;
    (t6690, t6691, t6692, t6694, t6695, t6697)
}
