//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1630/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1630<F: Float>(t11202: F, t996: F, t3325: F, t999: F, t1079: F, t3043: F, t378: F, t3042: F, t993: F) -> (F, F, F, F, F) {
    let t11203 = t996 * t11202;
    let t11206 = t999 * t3325;
    let t11207 = t1079 * t11206;
    let t11210 = t3043 * t378;
    let t11213 = t3042 * t993;
    (t11203, t11206, t11207, t11210, t11213)
}
