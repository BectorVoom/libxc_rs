//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1262/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1262<F: Float>(t10143: F, t1408: F, t1520: F, t254: F, t1527: F, t776: F, t1493: F, t1649: F, t2098: F, t671: F, t12461: F, t7939: F) -> (F, F, F, F, F, F, F) {
    let t98064 = t10143 * t1408;
    let t98279 = t1520 * t254;
    let t98960 = t1527 * t776;
    let t98975 = t1493 * t254;
    let t100688 = t10143 * t1649;
    let t100993 = t2098 * t671;
    let t101138 = t7939 * t12461;
    (t98064, t98279, t98960, t98975, t100688, t100993, t101138)
}
