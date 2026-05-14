//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1201/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1201<F: Float>(t24177: F, t2123: F, t6375: F, t6363: F, t920: F, t2115: F, t6188: F, t6189: F, t1569: F, t2590: F, t494: F, t5: F, t7: F, t2106: F, t2834: F, t2201: F, t6263: F, t785: F, t938: F) -> (F, F, F, F, F, F, F) {
    let t24178 = 0.48787202696913915093e-3 * t24177;
    let t24208 = t2123 * t6375;
    let t24209 = t920 * t6363;
    let t24442 = t6188 * t6189 * t2115;
    let t24447 = t24442 * t2590 * t1569 * t5 * t7 * t494;
    let t24452 = t2834 * t2106;
    let t24453 = 0.19043987679069580388e-1 * t24452;
    let t24463 = t2201 * t785 * t6263 * t938;
    (t24178, t24208, t24209, t24442, t24447, t24453, t24463)
}
