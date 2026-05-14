//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 319/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk319<F: Float>(t1260: F, t1262: F, t286: F, t1242: F, t1249: F, t1251: F, t1255: F, t423: F) -> (F, F, F) {
    let t1263 = t1260 * t1262;
    let t1264 = t286 * t1263;
    let t1267 = -t1242 * t423 / 72.0 + t1249 + t1251 * t1255 / 576.0 - t1251 * t1264 / 192.0;
    (t1263, t1264, t1267)
}
