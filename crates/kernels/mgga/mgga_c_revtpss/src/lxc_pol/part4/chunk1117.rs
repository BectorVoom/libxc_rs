//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1117/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1117<F: Float>(t15234: F, t973: F, t2962: F, t4673: F, t11452: F, t1621: F, t2944: F, t4708: F, t972: F, t1634: F, t3006: F, t2988: F, t4711: F, t3014: F, t4707: F, t11450: F, t11461: F, t11466: F, t11554: F, t15100: F, t15103: F, t15104: F, t2945: F, t2968: F, t2987: F, t3012: F, t4690: F, t4712: F, t965: F) -> (F,) {
    let t15235 = t15234 * t973;
    let t15238 = t4673 * t2962;
    let t15241 = t1621 * t11452;
    let t15242 = t15241 * t2944;
    let t15249 = t4708 * t972;
    let t15252 = t1634 * t3006;
    let t15255 = t4711 * t2988;
    let t15258 = t4707 * t3014;
    let t15259 = t15258 * t972;
    let t15262 = -t15100 + t15103 - 2.0 * t15104 * t2945 + 0.5848223622634646207e0 * t965 * t15235 + 0.32163958997385070134e2 * t2968 * t15238 + 0.2069040516770936012e4 * t11450 * t15242 - 0.23392894490538584828e1 * t11554 * t4690 + 0.34631718211362927518e2 * t11461 * t4712 - 0.23392894490538584828e1 * t2987 * t15249 - 0.11696447245269292414e1 * t2987 * t15252 - 0.10389515463408878255e3 * t11466 * t15255 + 0.34631718211362927518e2 * t3012 * t15259;
    (t15262,)
}
