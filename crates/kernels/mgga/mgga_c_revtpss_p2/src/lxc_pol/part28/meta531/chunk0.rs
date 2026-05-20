//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1972/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1972<F: Float>(t2007: F, t4292: F, t670: F, t7883: F, t1843: F, t7002: F, t651: F, t2322: F, t7742: F, t4254: F, t1310: F, t7741: F) -> (F, F, F, F, F, F, F) {
    let t28050 = t2007 * t4292;
    let t28053 = t7883 * t670;
    let t28056 = t1843 * t7002;
    let t28058 = F::new(2.0) * t651 * t28056;
    let t28060 = F::new(2.0) * t2322 * t7742;
    let t28062 = F::new(2.0) * t4254 * t7742;
    let t28063 = t1310 * t7741;
    (t28050, t28053, t28056, t28058, t28060, t28062, t28063)
}
