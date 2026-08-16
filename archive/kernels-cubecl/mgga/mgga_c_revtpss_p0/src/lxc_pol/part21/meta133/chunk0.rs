//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 857/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk857<F: Float>(t3147: F, t365: F, t3144: F, t3141: F, t1043: F) -> (F, F, F, F) {
    let t3148 = t365 * t3147;
    let t3149 = t3144 * t3148;
    let t3150 = t3141 * t3149;
    let t3151 = t1043 * t1043;
    (t3148, t3149, t3150, t3151)
}
