//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1149/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1149<F: Float>(t32269: F, t9492: F, t3785: F, t4237: F, t1415: F, t4226: F, t32247: F, t32249: F, t32251: F, t32253: F, t32256: F, t32258: F, t32262: F, t32264: F, t32267: F, t497: F, sigma0: F) -> (F, F, F, F, F) {
    let t32270 = t32269 * t9492;
    let t32272 = t3785 * t4237;
    let t32274 = t1415 * t4226;
    let t32276 = t32247 / 9.0 - 19.0 / 72.0 * t32249 - t32251 / 288.0 + t32253 / 128.0 + t32256 / 3.0 - t32258 / 12.0 + t32262 / 8.0 - t32264 / 3.0 + t32267 / 12.0 - t32270 / 8.0 - t32272 / 64.0 - t32274 / 12.0;
    let t32277 = sigma0 * t497;
    (t32270, t32272, t32274, t32276, t32277)
}
