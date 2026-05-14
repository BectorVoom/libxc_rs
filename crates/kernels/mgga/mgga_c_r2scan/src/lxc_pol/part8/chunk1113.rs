//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1113/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1113<F: Float>(t2110: F, t2162: F, t2304: F, t6162: F, t2168: F, t5135: F, t10979: F, t110: F, t20420: F, t19862: F, t1605: F, t6188: F, t6189: F, t481: F, t5: F, t7: F) -> (F, F, F, F, F, F) {
    let t20769 = 0.25059275625254849634e-3 * t2304 * t2110 * t2162 * t6162;
    let t20773 = t5135 * t2168;
    let t20791 = t20420 * t10979 * t110;
    let t20792 = t20791 * t19862;
    let t20818 = t6188 * t6189 * t1605;
    let t20820 = t5 * t7 * t481;
    (t20769, t20773, t20791, t20792, t20818, t20820)
}
