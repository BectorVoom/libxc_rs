//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 966/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk966<F: Float>(t1082: F, t23964: F, t23640: F, t378: F, t12079: F, t1668: F, t3302: F, t357: F, t19572: F, t4982: F, t6299: F, t4893: F) -> (F, F, F, F, F) {
    let t24075 = t1082 * t23964;
    let t24078 = t378 * t23640;
    let t24079 = t24078 * t12079;
    let t24083 = t3302 * t1668 * t357;
    let t24084 = t19572 * t24083;
    let t24089 = t4982 * t6299;
    let t24090 = t4893 * t24089;
    (t24075, t24078, t24079, t24084, t24090)
}
