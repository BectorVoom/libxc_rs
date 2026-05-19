//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 722/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk722<F: Float>(t162: F, t9348: F, t9361: F, t187: F, t2490: F, t737: F, t2492: F, t744: F) -> (F, F, F, F) {
    let t9363 = (t9348 + t9361) * t162;
    let t9365 = F::cast_from(0.19751673498613801407e-1_f64) * t9363 * t187;
    let t9367 = F::new(1.0) / t2490 / t737;
    let t9368 = t2492 * t744;
    (t9363, t9365, t9367, t9368)
}
