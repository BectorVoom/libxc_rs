//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 960/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk960<F: Float>(t11550: F, t3262: F, t3263: F, t2333: F, t983: F, t795: F, t3276: F, t3275: F, t2867: F, t792: F, t158: F, t955: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11552 = t3262 * t3263 * t11550;
    let t11553 = F::new(3.0) / F::new(4.0) * t11552;
    let t11554 = t2333 * t983;
    let t11555 = t11554 * t795;
    let t11556 = t3276 * t11555;
    let t11557 = t3275 * t11556;
    let t11558 = F::new(5.0) / F::new(16.0) * t11557;
    let t11559 = t2867 * t792;
    let t11560 = t3276 * t11559;
    let t11561 = t3275 * t11560;
    let t11562 = F::new(5.0) / F::new(16.0) * t11561;
    let t11563 = t158 * t955;
    (t11552, t11553, t11554, t11555, t11556, t11557, t11558, t11559, t11560, t11561, t11562, t11563)
}
