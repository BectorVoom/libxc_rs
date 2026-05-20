//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2455/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2455<F: Float>(t3115: F, t3119: F, t42793: F, t11688: F, t11922: F, t4892: F, t11249: F, t3151: F, t11722: F, t3188: F, t3046: F, t3316: F, t4891: F) -> (F, F, F, F, F) {
    let t42795 = t3115 * t42793 * t3119;
    let t42798 = t4892 * t11922 * t11688;
    let t42804 = t3151 * t11249;
    let t42816 = t3188 * t11722;
    let t42830 = t3046 * t3316 * t4891;
    (t42795, t42798, t42804, t42816, t42830)
}
