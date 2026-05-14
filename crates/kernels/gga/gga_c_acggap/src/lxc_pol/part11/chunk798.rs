//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 798/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk798<F: Float>(t2028: F, t7599: F, t2048: F, t2052: F, t7600: F, t154: F, t360: F, t7322: F, t7326: F, t1988: F, t7784: F, t377: F, t7613: F) -> (F, F, F, F, F, F) {
    let t30307 = t7599 * t2028;
    let t30308 = t30307 * t2048;
    let t30310 = t7600 * t2052;
    let t30314 = t7322 * t154 * t360 * t7326;
    let t30316 = t1988 * t7784;
    let t30318 = t377 * t7613;
    (t30307, t30308, t30310, t30314, t30316, t30318)
}
