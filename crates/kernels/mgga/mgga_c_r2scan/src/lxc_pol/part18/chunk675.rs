//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 675/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk675<F: Float>(t114: F, t5086: F, t133: F, t1541: F, t146: F, t1603: F, t2228: F, t2158: F, t147: F, t2182: F, t1591: F, t2132: F) -> (F, F, F, F, F, F, F) {
    let t5087 = t114 * t5086;
    let t5094 = t1541 * t133;
    let t5095 = t146 * t5094;
    let t5100 = t2228 * t1603;
    let t5101 = t5100 * t2158;
    let t5103 = t2182 * t147;
    let t5108 = t1591 * t2132;
    (t5087, t5094, t5095, t5100, t5101, t5103, t5108)
}
