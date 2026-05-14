//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 862/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk862<F: Float>(t1004: F, t3829: F, t1035: F, t14255: F, t452: F, t3055: F, t3058: F, t1261: F, t848: F, t12254: F, t3828: F, t864: F, t3088: F, t407: F, t441: F, t12730: F, t180: F, t3037: F) -> (F, F, F, F, F, F, F, F) {
    let t14486 = t1004 * t3829;
    let t14490 = 0.39512695097613069591e1 * t1035 * t452 * t14255;
    let t14491 = t3055 * t3058;
    let t14495 = t848 * t1261;
    let t14501 = 0.52683593463484092788e1 * t1035 * t452 * t12254;
    let t14503 = t1035 * t3828 * t864;
    let t14518 = t3088 * t441 * t864 * t407;
    let t14525 = t12730 * t180 * t3037 * t407;
    (t14486, t14490, t14491, t14495, t14501, t14503, t14518, t14525)
}
