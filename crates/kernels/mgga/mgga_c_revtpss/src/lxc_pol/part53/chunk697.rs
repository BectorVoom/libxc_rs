//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 697/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk697<F: Float>(t225: F, t494: F, t8190: F, t1769: F, t2142: F, t7637: F, t1774: F) -> (F, F, F, F) {
    let t8192 = t8190 * t225 * t494;
    let t8197 = t2142 * t1769;
    let t8198 = t7637 * t8197;
    let t8201 = t2142 * t1774;
    (t8192, t8197, t8198, t8201)
}
