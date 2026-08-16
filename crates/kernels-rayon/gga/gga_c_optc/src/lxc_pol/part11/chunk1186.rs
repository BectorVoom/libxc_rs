//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1186/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1186(t11975: f64, t17688: f64, t3116: f64, t1111: f64, t17666: f64, t24: f64, t17927: f64, t861: f64, t11937: f64, t15622: f64, t1128: f64, t17704: f64, t8913: f64) -> (f64, f64, f64, f64, f64) {
    let t54245 = t3116 * t11975 * t17688;
    let t54248 = t1111 * t24 * t17666;
    let t54252 = t17927 * t861;
    let t54261 = t11937 * t15622;
    let t54268 = t8913 * t1128 * t17704;
    (t54245, t54248, t54252, t54261, t54268)
}
