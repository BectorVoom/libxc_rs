//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 751/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk751<F: Float>(t1615: F, t784: F, t2150: F, t2147: F, t1267: F, t512: F, t57: F, t2158: F, t1415: F, t511: F, t2162: F, t2164: F) -> (F, F, F, F, F) {
    let t6398 = t784 * t1615;
    let t6399 = t6398 * t2150;
    let t6400 = t2147 * t6399;
    let t6407 = t512 * t1267 * t57;
    let t6408 = t6407 * t2158;
    let t6412 = t1415 * t511;
    let t6415 = F::cast_from(0.89443204944342177673e-3_f64) * t6412 * t2162 * t2164;
    (t6398, t6400, t6407, t6408, t6415)
}
