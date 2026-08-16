//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1181/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1181<F: Float>(t11880: F, t11881: F, t1289: F, t3358: F, t8395: F, t11036: F, t8370: F, t8373: F, t1070: F, t23353: F, t11033: F, t2391: F) -> (F, F, F, F, F, F) {
    let t40828 = t11880 * t11881 * t1289;
    let t40830 = t3358 * t8395;
    let t40833 = t11036 * t8370;
    let t40835 = t11036 * t8373;
    let t40837 = t23353 * t1070;
    let t40840 = t11033 * t2391;
    (t40828, t40830, t40833, t40835, t40837, t40840)
}
