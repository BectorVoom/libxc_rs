//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 660/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk660<F: Float>(t225: F, t7274: F, t2022: F, t213: F, t1032: F, t555: F, t1426: F) -> (F, F, F, F) {
    let t7275 = t7274 * t225;
    let t7279 = t213 * t2022;
    let t7282 = t555 * t1032;
    let t7283 = t7282 * t1426;
    (t7275, t7279, t7282, t7283)
}
