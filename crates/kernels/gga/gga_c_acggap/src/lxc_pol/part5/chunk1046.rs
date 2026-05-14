//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1046/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1046<F: Float>(t1181: F, t20992: F, t3361: F, t4267: F, t1163: F, t1165: F, t4210: F, t5852: F, t5574: F, t997: F, t1008: F, t1180: F, t1531: F, t1532: F, t15982: F, t20961: F, t20963: F, t20969: F, t20972: F, t20985: F, t20987: F, t406: F, t4417: F, t4463: F, t5752: F, t929: F) -> (F,) {
    let t20995 = t3361 * t1181 * t4267 * t20992;
    let t20999 = t1163 * t1165 * t5852 * t4210;
    let t21001 = t997 * t5574;
    let t21003 = t1008 * t5574;
    let t21005 = -0.68598428988911579156e-2 * t20961 + 0.34299214494455789578e-2 * t1531 * t1181 * t1532 * t20963 * t406 - 0.32012600194825403606e-1 * t20969 - 0.51448821741683684368e-2 * t1180 * t1165 * t4417 * t20972 + 0.17149607247227894789e-2 * t1531 * t1181 * t1532 * t5752 * t929 - 0.25724410870841842183e-2 * t15982 - 0.34299214494455789578e-2 * t20985 - 0.68598428988911579156e-1 * t4463 * t1181 * t4267 * t20987 - 0.13719685797782315831e-1 * t20995 + 0.42874018118069736972e-3 * t20999 + 0.4801890029223810541e-1 * t21001 - 0.10289764348336736874e-1 * t21003;
    (t21005,)
}
