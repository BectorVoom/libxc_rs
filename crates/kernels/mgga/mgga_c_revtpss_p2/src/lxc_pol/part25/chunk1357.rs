//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1357/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1357<F: Float>(t116: F, t25832: F, t572: F, t670: F, t2371: F, t26123: F, t1459: F, t26130: F, t4158: F, t7331: F, t7334: F, t13232: F, t2042: F) -> (F, F, F, F, F, F) {
    let t95137 = t116 * t25832;
    let t95140 = F::cast_from(18.0_f64) * t572 * t95137 * t670;
    let t95143 = F::cast_from(18.0_f64) * t572 * t26123 * t2371;
    let t95147 = F::cast_from(9.0_f64) * t1459 * t26130;
    let t95149 = F::cast_from(18.0_f64) * t4158 * t7331;
    let t95153 = F::cast_from(9.0_f64) * t4158 * t7334;
    let t95157 = F::cast_from(3.0_f64) * t13232 * t2042;
    (t95140, t95143, t95147, t95149, t95153, t95157)
}
