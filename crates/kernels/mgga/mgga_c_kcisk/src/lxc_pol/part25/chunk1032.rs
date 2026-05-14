//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1032/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1032<F: Float>(t18325: F, t2012: F, t2020: F, t5014: F, t5492: F, t6763: F, t12163: F, t12195: F, t18254: F, t18260: F, t18264: F, t18270: F, t18272: F, t18275: F, t18319: F, t18323: F, t2016: F, t2638: F, t5471: F, t5488: F, t5494: F, t5503: F, t7581: F, t7591: F, t7606: F, t7611: F, t7615: F, t7619: F, t788: F) -> (F, F, F) {
    let t18326 = t2012 * t18325;
    let t18327 = t5014 * t2020;
    let t18328 = t6763 * t5492;
    let t18329 = t18327 * t18328;
    let t18332 = 0.41978507004413740163e-1 * t18254 + 0.89953943580886586067e-2 * t12195 * t2638 - 0.35981577432354634426e-1 * t5471 * t7619 - 0.47975436576472845902e-1 * t18260 * t2016 - t18264 - 0.23987718288236422951e-1 * t7591 * t5503 - 0.31983624384315230601e-1 * t7591 * t5488 + t18270 + 0.17990788716177317213e-1 * t18272 * t2016 + 0.59969295720591057378e-2 * t18275 + 0.89953943580886586067e-2 * t7581 * t5503 + 0.11993859144118211476e-1 * t7581 * t5488 - 0.17990788716177317213e-1 * t5471 * t7611 - 0.35981577432354634426e-1 * t5471 * t7615 + 0.23987718288236422952e-1 * t5471 * t7606 - 0.17990788716177317213e-1 * t7581 * t5494 + 0.2698618307426597582e-1 * t18319 * t788 + t18323 - 0.59969295720591057378e-2 * t12163 + 0.35981577432354634426e-1 * t18326 * t18329;
    (t18326, t18328, t18332)
}
