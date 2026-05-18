//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1163/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1163<F: Float>(t10918: F, t12570: F, t3262: F, t10610: F, t12414: F, t3275: F, t8601: F, t114: F, t481: F, t97: F, t12415: F, t2847: F, t3574: F) -> (F, F, F, F, F) {
    let t42908 = F::new(3.0) / F::new(4.0) * t3262 * t10918 * t12570;
    let t42911 = F::new(3.0) / F::new(2.0) * t10610 * t10918 * t12414;
    let t42914 = t3275 * t10918 * t8601 / F::new(4.0);
    let t42916 = t97 * t481 * t114;
    let t42918 = F::new(3.0) / F::new(2.0) * t42916 * t12415;
    let t42919 = t3574 * t2847;
    (t42908, t42911, t42914, t42918, t42919)
}
