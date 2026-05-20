//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1033/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1033<F: Float>(t32268: F, t8590: F, t1381: F, t32247: F, t552: F, t1385: F, t8584: F) -> (F, F, F, F, F) {
    let t32269 = t32268 * t8590;
    let t32270 = t32269 * t1381;
    let t32272 = t32247 * t8590;
    let t32273 = t32272 * t552;
    let t32275 = t8584 * t1385;
    (t32269, t32270, t32272, t32273, t32275)
}
