//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1084/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1084<F: Float>(t17402: F, t218: F, t5555: F, t679: F, t1878: F, t1885: F, t1889: F, t5568: F, t675: F, t5572: F, t16194: F, t213: F, t778: F) -> (F, F, F, F, F, F, F) {
    let t17403 = F::new(0.13490888888888888889e1) * t17402;
    let t17405 = t218 * t5555 * t679;
    let t17408 = t218 * t1878 * t1885;
    let t17411 = t218 * t1878 * t1889;
    let t17414 = t218 * t675 * t5568;
    let t17417 = t218 * t675 * t5572;
    let t17432 = F::new(1.0) / t213 / t16194 / t778 / F::new(96.0);
    (t17403, t17405, t17408, t17411, t17414, t17417, t17432)
}
