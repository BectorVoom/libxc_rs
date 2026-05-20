//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1011/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1011<F: Float>(t24214: F, t24217: F, t24219: F, t24223: F, t24264: F, t24326: F, t24329: F, t24468: F, t24472: F, t24475: F, t24478: F, t24492: F) -> F {
    let t24769 = -t24264 + t24326 + t24329 - t24478 - t24492 + t24472 - t24468 - t24475 - t24219 + t24223 - t24214 + t24217;
    t24769
}
