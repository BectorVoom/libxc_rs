//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 956/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk956<F: Float>(t10863: F, t10866: F, t10901: F, t11017: F, t11002: F, t1115: F, t792: F, t2867: F, t481: F, t3574: F, t2333: F, t910: F) -> (F, F, F, F, F, F, F, F) {
    let t11432 = F::cast_from(0.28914548798370980346e-3_f64) * t10863;
    let t11433 = F::cast_from(0.42683466926433871473e0_f64) * t10866;
    let t11444 = F::cast_from(0.45022119329691164871e0_f64) * t10901;
    let t11454 = F::cast_from(0.39032073591371545778e-3_f64) * t11017;
    let t11465 = t11002 * t1115 * t792;
    let t11475 = t2867 * t481;
    let t11486 = t3574 * t792;
    let t11496 = t2333 * t910;
    (t11432, t11433, t11444, t11454, t11465, t11475, t11486, t11496)
}
