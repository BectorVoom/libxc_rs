//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 430/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk430<F: Float>(t1719: F, t720: F, t748: F, t234: F, t218: F, t716: F) -> (F, F, F, F) {
    let t1813 = t720 * t1719;
    let t1814 = t748 * t1813;
    let t1816 = F::cast_from(0.17315859105681463759e2_f64) * t234 * t1814;
    let t1818 = F::new(1.0) / t716 / t218;
    (t1813, t1814, t1816, t1818)
}
