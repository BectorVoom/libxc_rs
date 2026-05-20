//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1201/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1201<F: Float>(t1401: F, t7264: F, t1405: F, t2019: F, t545: F, t64: F) -> (F, F, F) {
    let t7265 = t7264 * t1401;
    let t7267 = t2019 * t1405;
    let t7268 = F::cast_from(0.20007875121765877254e-2_f64) * t7267;
    let t7269 = t545 * t64;
    (t7265, t7268, t7269)
}
