//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 705/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk705<F: Float>(t213: F, t5744: F, t4086: F, t2242: F, t38: F, t1925: F) -> (F, F, F, F) {
    let t5745 = t213 * t5744;
    let t5755 = t213 * t4086;
    let t6954 = t2242 * t38;
    let t6957 = t38 * t1925;
    (t5745, t5755, t6954, t6957)
}
