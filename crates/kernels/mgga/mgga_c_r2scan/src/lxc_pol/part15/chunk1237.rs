//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1237/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1237<F: Float>(t11036: F, t8370: F, t8373: F, t1070: F, t23353: F, t37041: F, t11033: F, t2391: F, t37031: F, t8367: F, t3366: F, t8355: F) -> (F, F, F, F, F, F, F) {
    let t40833 = t11036 * t8370;
    let t40835 = t11036 * t8373;
    let t40837 = t23353 * t1070;
    let t40839 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t37041;
    let t40840 = t11033 * t2391;
    let t40841 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t40840;
    let t40842 = t37031 * t8367;
    let t40844 = t8355 * t3366;
    (t40833, t40835, t40837, t40839, t40841, t40842, t40844)
}
