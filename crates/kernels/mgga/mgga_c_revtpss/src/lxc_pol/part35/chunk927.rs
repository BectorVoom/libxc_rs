//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 927/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk927<F: Float>(t11465: F, t23451: F, t3014: F, t981: F, t3011: F, t973: F, t1610: F, t19056: F, t4590: F, t6142: F, t15421: F, t6145: F) -> (F, F, F, F, F) {
    let t23452 = t11465 * t23451;
    let t23453 = t23452 * t3014;
    let t23455 = F::new(0.10389515463408878255e3) * t981 * t23453;
    let t23457 = t3011 * t23451 * t973;
    let t23459 = F::new(0.35089341735807877242e1) * t981 * t23457;
    let t23461 = F::new(3.0) * t19056 * t1610;
    let t23463 = F::new(3.0) * t4590 * t6142;
    let t23465 = F::new(0.48245938496077605201e2) * t15421 * t6145;
    (t23455, t23459, t23461, t23463, t23465)
}
