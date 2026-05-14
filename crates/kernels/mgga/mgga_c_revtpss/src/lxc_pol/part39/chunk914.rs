//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 914/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk914<F: Float>(t1312: F, t2179: F, t2181: F, t2322: F, t4254: F, t5523: F, t651: F, t8254: F, t8274: F, t8278: F, t8280: F, t3: F) -> (F, F, F) {
    let t8283 = 2.0 * t1312 * t8278 + 2.0 * t1312 * t8280 - 2.0 * t2179 * t2322 - 2.0 * t2179 * t4254 + 2.0 * t2181 * t2322 + 2.0 * t2181 * t5523 - 2.0 * t651 * t8254 - 2.0 * t651 * t8274;
    let t8284 = t3 * t8283;
    let t8289 = param_d * t8283;
    (t8283, t8284, t8289)
}
