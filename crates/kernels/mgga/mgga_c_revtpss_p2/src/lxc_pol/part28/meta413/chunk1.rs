//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1561/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1561<F: Float>(t15191: F, t15197: F, t11134: F, t11136: F, t11138: F, t11140: F, t11339: F, t11366: F, t11368: F, t11422: F, t11423: F, t15221: F, t15230: F) -> (F, F, F) {
    let t15322 = F::cast_from(0.34431666666666666666e0_f64) * t15191;
    let t15324 = F::cast_from(0.13892666666666666667e0_f64) * t15197;
    let t15337 = -t11422 - t11423 + F::cast_from(0.6311625e0_f64) * t15221 + F::cast_from(0.23154444444444444444e-1_f64) * t11339 - F::cast_from(0.34431666666666666666e0_f64) * t11138 - F::cast_from(0.45908888888888888888e0_f64) * t11134 + F::cast_from(0.17215833333333333333e0_f64) * t11140 + F::cast_from(0.11477222222222222222e0_f64) * t11136 - F::cast_from(0.23154444444444444444e0_f64) * t11366 + F::cast_from(0.69463333333333333333e-1_f64) * t11368 + F::cast_from(0.3529725e1_f64) * t15230;
    (t15322, t15324, t15337)
}
