//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1228/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1228<F: Float>(t2124: F, t29496: F, t39849: F, t11670: F, t29500: F, t10868: F, t2147: F, t9445: F, t10760: F, t30320: F, t30468: F, t6085: F) -> (F, F, F, F, F) {
    let t43654 = t39849 * t2124 * t29496;
    let t43657 = t11670 * t2124 * t29500;
    let t43660 = t2147 * t10868 * t9445;
    let t43664 = t2147 * t10760 * t30320;
    let t43667 = t6085 * t10760 * t30468;
    (t43654, t43657, t43660, t43664, t43667)
}
