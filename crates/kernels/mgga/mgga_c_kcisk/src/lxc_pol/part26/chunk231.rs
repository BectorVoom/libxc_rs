//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 231/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk231<F: Float>(t1139: F, t285: F, t1033: F, t1037: F, t1040: F, t1043: F, t1046: F, t1051: F) -> (F, F) {
    let t1140 = t285 * t1139;
    let t1147 = 0.1875e0 * t1033 - 0.1875e0 * t1037 - 0.375e0 * t1040 - 0.4046875e-1 * t1043 + 0.4046875e-1 * t1046 + 0.161875e0 * t1051;
    (t1140, t1147)
}
