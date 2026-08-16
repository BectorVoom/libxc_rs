//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 434/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk434<F: Float>(t2042: F, t572: F, t2040: F, t573: F, t10: F, t17: F, t576: F, t580: F, t15: F, t22: F, t11: F, t14: F) -> (F, F, F, F, F) {
    let t2044 = F::cast_from(3.0_f64) * t572 * t2042;
    let t2045 = t2040 * t573 + t2044;
    let t2219 = F::cast_from(2.0_f64) * t10 * t17;
    let t2221 = F::cast_from(8.0_f64) * t576 * t580;
    let t2223 = F::cast_from(6.0_f64) * t15 * t22;
    let t2224 = t11 * t14;
    (t2045, t2219, t2221, t2223, t2224)
}
