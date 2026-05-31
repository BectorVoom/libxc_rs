//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2031/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2031<F: Float>(t12984: F, t7613: F, t12966: F, t2138: F, t12851: F, t2134: F, t3567: F, t8945: F, t26894: F, t29199: F, t3596: F, t37885: F) -> (F, F, F, F, F, F) {
    let t97288 = t7613 * t12984;
    let t97292 = t12966 * t2138;
    let t97296 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t2134 * t12851;
    let t97304 = t3567 * t8945;
    let t97308 = t26894 * t29199;
    let t97312 = t37885 * t3596;
    (t97288, t97292, t97296, t97304, t97308, t97312)
}
