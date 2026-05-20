//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2727/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2727<F: Float>(t11024: F, t1580: F, t689: F, t10981: F, t1579: F, t22: F, t868: F, t15060: F, t2435: F, t14982: F, t2465: F, t2470: F) -> (F, F, F, F) {
    let t50174 = t689 * t11024 * t1580;
    let t50178 = t10981 * t868 * t1579 * t22;
    let t50183 = t2435 * t15060;
    let t50184 = F::cast_from(0.21951497276451705329e-1_f64) * t50183;
    let t50186 = t2465 * t14982 * t2470;
    (t50174, t50178, t50184, t50186)
}
