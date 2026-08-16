//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1651/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1651<F: Float>(t2311: F, t76: F, t10298: F, t38: F, t2248: F, t77: F, t84: F, t2247: F, t607: F) -> (F, F, F, F) {
    let t25146 = t76 * t2311;
    let t25150 = t10298 * t38;
    let t25159 = t77 * t84 * t2248;
    let t25162 = t2247 * t607;
    (t25146, t25150, t25159, t25162)
}
