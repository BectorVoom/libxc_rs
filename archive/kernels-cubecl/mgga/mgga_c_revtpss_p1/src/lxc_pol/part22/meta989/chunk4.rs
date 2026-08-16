//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3364/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3364<F: Float>(t42518: F, t52011: F, t60927: F, t4606: F, t2897: F, t51957: F, t52110: F) -> (F, F, F, F) {
    let t63393 = t52011 * t42518 * t60927;
    let t63395 = t4606 * t4606;
    let t63396 = t2897 * t63395;
    let t63399 = t51957 * t52110 * t60927;
    (t63393, t63395, t63396, t63399)
}
