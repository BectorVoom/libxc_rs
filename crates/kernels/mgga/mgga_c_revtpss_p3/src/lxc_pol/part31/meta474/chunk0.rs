//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1740/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1740<F: Float>(t4003: F, t6843: F, t2723: F, t6016: F, t197: F, t531: F, t2013: F) -> (F, F, F, F) {
    let t23037 = t4003 * t6843;
    let t23160 = t2723 * t6016;
    let t25081 = t197 * t531;
    let t25082 = t2013 * t25081;
    (t23037, t23160, t25081, t25082)
}
