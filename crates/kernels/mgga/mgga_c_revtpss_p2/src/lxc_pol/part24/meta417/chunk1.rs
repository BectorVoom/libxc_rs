//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1364/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1364<F: Float>(t42872: F, t43351: F, t1035: F, t42859: F, t342: F, t357: F, t3057: F, t4980: F, t11200: F, t3286: F, t4995: F, t3143: F) -> (F, F, F, F, F, F, F) {
    let t43352 = t43351 * t42872;
    let t43400 = t42859 * t1035;
    let t43401 = t342 * t43400;
    let t43402 = t43351 * t357;
    let t43438 = t3057 * t4980;
    let t43446 = t11200 * t3286;
    let t43456 = t3057 * t4995;
    let t43471 = t42859 * t3143;
    (t43352, t43401, t43402, t43438, t43446, t43456, t43471)
}
