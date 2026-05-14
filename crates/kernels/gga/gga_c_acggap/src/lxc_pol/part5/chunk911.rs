//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 911/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk911<F: Float>(t14047: F, t4904: F, t1101: F, t1165: F, t1586: F, t3361: F, t16548: F, t3194: F, t530: F, t12586: F, t5147: F, t1017: F, t1163: F, t1539: F, t4313: F, t12589: F, t5152: F) -> (F, F, F, F, F, F) {
    let t16765 = t14047 * t4904;
    let t16769 = t3361 * t1165 * t1586 * t1101;
    let t16779 = t3194 * t1165 * t530 * t16548;
    let t16781 = t12586 * t5147;
    let t16786 = t1163 * t1165 * t4313 * t1539 * t1017;
    let t16788 = t12589 * t5152;
    (t16765, t16769, t16779, t16781, t16786, t16788)
}
