//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1197/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1197<F: Float>(t25197: F, t26092: F, t3: F, t2042: F, t4158: F, t1459: F, t7331: F, t7334: F, t1936: F, t2327: F, t572: F, t116: F, t7002: F, param_d: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26093 = t25197 + t26092;
    let t26094 = t3 * t26093;
    let t26106 = param_d * t26093;
    let t26115 = F::new(3.0) * t4158 * t2042;
    let t26117 = F::new(12.0) * t1459 * t7331;
    let t26119 = F::new(6.0) * t1459 * t7334;
    let t26120 = t2327 * t1936;
    let t26122 = F::new(6.0) * t572 * t26120;
    let t26123 = t116 * t7002;
    (t26093, t26094, t26106, t26115, t26117, t26119, t26120, t26122, t26123)
}
