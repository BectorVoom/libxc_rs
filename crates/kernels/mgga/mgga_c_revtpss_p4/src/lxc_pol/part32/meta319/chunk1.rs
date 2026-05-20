//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1235/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1235<F: Float>(t1025: F, t11817: F, t271: F, t2857: F, t283: F, t3298: F, t994: F, t4891: F, t3154: F, t999: F, t1086: F, t3046: F) -> (F, F, F, F, F, F) {
    let t11818 = t1025 * t11817;
    let t11821 = F::new(1.0) / t271 / t2857;
    let t11852 = F::new(1.0) / t283 / t2857;
    let t11858 = t994 * t3298;
    let t11859 = t11858 * t4891;
    let t11860 = t3154 * t999;
    let t11865 = t3046 * t1086;
    (t11818, t11821, t11852, t11859, t11860, t11865)
}
