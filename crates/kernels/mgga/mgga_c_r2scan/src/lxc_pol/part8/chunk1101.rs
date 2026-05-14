//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1101/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1101<F: Float>(t2183: F, t2597: F, t2102: F, t572: F, t565: F, t120: F, t135: F, t6329: F, t1391: F, t20: F, t481: F, t506: F, t6068: F, t6159: F, t2146: F, t2182: F) -> (F, F, F, F, F, F, F) {
    let t19845 = t2183 * t2597;
    let t19851 = t572 * t2102;
    let t19852 = t565 * t19851;
    let t19858 = 0.49323117337212473701e1 * t120 * t6329 * t135;
    let t19862 = t506 * t20 * t1391 * t481;
    let t19863 = t6159 * t6068 * t19862;
    let t19865 = t2182 * t2146;
    (t19845, t19851, t19852, t19858, t19862, t19863, t19865)
}
