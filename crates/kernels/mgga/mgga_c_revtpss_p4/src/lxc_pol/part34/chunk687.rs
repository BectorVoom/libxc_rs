//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 687/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk687<F: Float>(t2247: F, t6957: F, t43: F, t48: F, t624: F, t116: F, t1931: F) -> (F, F, F, F) {
    let t6958 = t2247 * t6957;
    let t6968 = t43 * t48;
    let t6971 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t624;
    let t6985 = t1931 * t116;
    (t6958, t6968, t6971, t6985)
}
