//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1152/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1152<F: Float>(t11860: F, t19501: F, t3117: F, t19611: F, t3095: F, t3092: F, t19414: F, t247: F, t3116: F, t1651: F, t4866: F, t1045: F, t2857: F, t4181: F, t2852: F, t11703: F) -> (F, F, F, F, F, F, F) {
    let t20074 = t19501 * t11860;
    let t20075 = t3117 * t20074;
    let t20078 = t19611 * t3095;
    let t20079 = t3092 * t20078;
    let t20083 = t247 * t3116 * t19414;
    let t20089 = t1651 * t4866;
    let t20090 = t20089 * t1045;
    let t20091 = t3117 * t20090;
    let t20094 = t1651 * t2857;
    let t20095 = t20094 * t4181;
    let t20096 = t3092 * t20095;
    let t20099 = t1651 * t2852;
    let t20100 = t20099 * t4181;
    let t20101 = t11703 * t20100;
    (t20075, t20079, t20083, t20089, t20091, t20096, t20101)
}
