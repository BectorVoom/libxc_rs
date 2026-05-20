//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 797/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk797<F: Float>(t1317: F, t3855: F, t4029: F, t1333: F, t3863: F, t27: F, t583: F, t521: F, t19: F, t596: F, t182: F, t2490: F) -> (F, F, F, F, F, F) {
    let t9404 = t1317 * t3855;
    let t9405 = F::new(12.0) * t9404;
    let t9406 = t1317 * t4029;
    let t9407 = F::new(24.0) * t9406;
    let t9408 = t3863 * t1333;
    let t9409 = F::new(96.0) * t9408;
    let t9410 = t583 * t27;
    let t9411 = t9410 * t521;
    let t9412 = F::new(240.0) * t9411;
    let t9413 = t19 * t596;
    let t9415 = F::new(120.0) * t9413 * t521;
    let t9417 = F::new(1.0) / t2490 / t182;
    (t9405, t9407, t9409, t9412, t9415, t9417)
}
