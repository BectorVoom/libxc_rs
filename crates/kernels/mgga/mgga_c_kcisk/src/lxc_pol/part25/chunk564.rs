//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 564/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk564<F: Float>(t1899: F, t5068: F, t1873: F, t1869: F, t140: F, t1797: F, t3737: F) -> (F, F, F, F) {
    let t5069 = t1899 * t5068;
    let t5070 = t1873 * t5069;
    let t5071 = t1869 * t5070;
    let t5074 = t140 * t3737 * t1797;
    (t5069, t5070, t5071, t5074)
}
