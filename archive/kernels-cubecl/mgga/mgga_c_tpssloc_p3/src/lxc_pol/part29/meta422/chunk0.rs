//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1704/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1704<F: Float>(t1385: F, t5353: F, t3887: F, t16413: F, t539: F, t225: F, t5217: F, t1834: F, t3752: F, t1323: F, t5318: F, t16122: F, t562: F) -> (F, F, F, F, F, F, F) {
    let t16452 = t5353 * t1385;
    let t16453 = t3887 * t16452;
    let t16458 = t539 * t16413;
    let t16460 = t5217 * t225;
    let t16463 = t3752 * t1834;
    let t16465 = t1323 * t5318;
    let t16468 = t16122 * t562;
    (t16452, t16453, t16458, t16460, t16463, t16465, t16468)
}
