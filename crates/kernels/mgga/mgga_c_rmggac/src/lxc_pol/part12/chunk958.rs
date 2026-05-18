//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 958/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk958<F: Float>(t1525: F, t1971: F, t209: F, t236: F, t476: F, t7453: F, t1212: F, t615: F, t1240: F, t1475: F, t1182: F, t570: F) -> (F, F, F, F) {
    let t40414 = t7453 * t1971 * t236 * t1525 * t476 * t209;
    let t40420 = t7453 * t1971 * t236 * t615 * t1212 * t209;
    let t40425 = t7453 * t1971 * t236 * t1475 * t1240;
    let t40427 = t570 * t1182;
    (t40414, t40420, t40425, t40427)
}
