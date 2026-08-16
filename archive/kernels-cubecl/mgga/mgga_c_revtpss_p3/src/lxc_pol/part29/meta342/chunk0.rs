//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1262/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1262<F: Float>(t3111: F, t3188: F, t3211: F, t3215: F, t1026: F, t371: F, t676: F, t1025: F, t271: F, t2857: F, t283: F, t3298: F, t994: F) -> (F, F, F, F, F, F) {
    let t11802 = t3188 * t3111;
    let t11814 = t3211 * t3215;
    let t11817 = t371 * t676 * t1026;
    let t11818 = t1025 * t11817;
    let t11821 = F::cast_from(1.0_f64) / t271 / t2857;
    let t11852 = F::cast_from(1.0_f64) / t283 / t2857;
    let t11858 = t994 * t3298;
    (t11802, t11814, t11818, t11821, t11852, t11858)
}
