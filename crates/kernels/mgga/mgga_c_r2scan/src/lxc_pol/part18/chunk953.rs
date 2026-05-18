//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 953/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk953<F: Float>(t2867: F, t481: F, t3263: F, t3262: F, t3617: F, t498: F) -> (F, F, F, F, F) {
    let t11475 = t2867 * t481;
    let t11476 = t3263 * t11475;
    let t11477 = t3262 * t11476;
    let t11478 = F::new(3.0) / F::new(4.0) * t11477;
    let t11479 = t498 * t3617;
    (t11475, t11476, t11477, t11478, t11479)
}
