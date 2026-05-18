//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1080/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1080<F: Float>(t17999: F, t197: F, t2030: F, t17928: F, t5951: F, t1478: F, t301: F, t154: F, t276: F, t655: F, t486: F, t779: F) -> (F, F, F, F, F, F, F) {
    let t18000 = t17999 * t197;
    let t18002 = t2030 * t2030;
    let t18008 = t17928 * t5951;
    let t18009 = t18008 * t197;
    let t18060 = t1478 * t301;
    let t18063 = t276 * t154 * t18060 * t655;
    let t18086 = t486 * t779;
    (t18000, t18002, t18008, t18009, t18060, t18063, t18086)
}
