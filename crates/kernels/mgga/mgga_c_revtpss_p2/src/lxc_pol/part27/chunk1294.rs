//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1294/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1294<F: Float>(t2371: F, t26123: F, t572: F, t1459: F, t26130: F, t4158: F, t7331: F, t7334: F, t13232: F, t2042: F, t28264: F, t2327: F, t7002: F) -> (F, F, F, F, F, F, F) {
    let t95143 = F::new(18.0) * t572 * t26123 * t2371;
    let t95147 = F::new(9.0) * t1459 * t26130;
    let t95149 = F::new(18.0) * t4158 * t7331;
    let t95153 = F::new(9.0) * t4158 * t7334;
    let t95157 = F::new(3.0) * t13232 * t2042;
    let t95160 = F::new(18.0) * t572 * t28264 * t2371;
    let t95163 = F::new(18.0) * t572 * t2327 * t7002;
    (t95143, t95147, t95149, t95153, t95157, t95160, t95163)
}
