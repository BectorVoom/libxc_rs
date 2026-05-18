//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1292/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1292<F: Float>(t1241: F, t21100: F, t7616: F, t1256: F, t30789: F, t29037: F, t5378: F, t20786: F, t26849: F, t29010: F, t5265: F, t20819: F, t7617: F) -> (F, F, F, F, F, F) {
    let t112307 = t1241 * t7616 * t21100;
    let t112322 = t30789 * t1256;
    let t112328 = t29037 * t5378;
    let t112334 = t26849 * t20786;
    let t112336 = t29010 * t5265;
    let t112339 = t20819 * t7617;
    (t112307, t112322, t112328, t112334, t112336, t112339)
}
