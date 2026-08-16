//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1700/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1700<F: Float>(t26153: F, t508: F, t2106: F, t530: F, t25865: F, t6977: F, t7348: F) -> (F, F, F) {
    let t26154 = t508 * t26153;
    let t26161 = t530 * t2106;
    let t26162 = t26161 * t25865;
    let t26169 = t7348 * t6977;
    (t26154, t26162, t26169)
}
