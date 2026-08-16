//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2222/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2222<F: Float>(t1204: F, t8190: F, t2142: F, t5284: F, t3153: F, t1276: F, t42859: F, t13038: F, t2149: F, t11249: F, t29157: F, t73: F) -> (F, F, F, F, F, F) {
    let t104465 = t1204 * t8190;
    let t104472 = t2142 * t5284;
    let t104473 = t104472 * t3153;
    let t104480 = t42859 * t1276;
    let t104482 = t2149 * t104480 * t13038;
    let t104483 = t29157 * t11249;
    let t104490 = t104472 * t73;
    (t104465, t104473, t104480, t104482, t104483, t104490)
}
