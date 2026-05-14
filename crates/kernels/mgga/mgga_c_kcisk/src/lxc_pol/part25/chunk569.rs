//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 569/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk569<F: Float>(t5134: F, t1849: F, t967: F, t1060: F, t158: F, t165: F, t173: F, t1809: F, t3293: F, t5111: F, t5114: F, t5117: F, t5122: F, t5125: F, t5128: F, t5129: F, t5130: F) -> (F, F, F, F) {
    let t5135 = 0.15538616723388920628e-3 * t5134;
    let t5136 = t967 * t1849;
    let t5137 = t5136 * t1060;
    let t5139 = 0.7026e-2 * t158 * t5111 - 0.1585e-2 * t165 * t5114 - 0.10082625e-4 * t173 * t5117 + t5122 - t5125 - t5128 - t5129 - 0.23911438650126355246e-1 * t5130 + 0.11955719325063177623e-1 * t1809 * t3293 + t5135 + 0.20718155631185227504e-3 * t5137;
    (t5135, t5136, t5137, t5139)
}
