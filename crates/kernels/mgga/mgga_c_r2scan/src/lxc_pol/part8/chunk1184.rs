//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1184/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1184<F: Float>(t2132: F, t5135: F, t10878: F, t545: F, t1598: F, t524: F, t6291: F, t1541: F, t20: F, t525: F, t128: F, t20094: F, t6188: F, t6195: F, t20137: F, t6209: F, t6213: F) -> (F, F, F, F, F, F, F) {
    let t22721 = t5135 * t2132;
    let t22731 = t545 * t10878;
    let t22744 = t524 * t1598 * t6291;
    let t22749 = t524 * t525 * t1541 * t20;
    let t22766 = t20094 * t128;
    let t22767 = t6188 * t22766;
    let t22768 = t22767 * t6195;
    let t22778 = t6209 * t20137 * t6213;
    (t22721, t22731, t22744, t22749, t22767, t22768, t22778)
}
