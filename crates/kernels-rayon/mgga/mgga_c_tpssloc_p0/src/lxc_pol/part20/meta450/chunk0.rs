//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1900/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1900(t1117: f64, t4782: f64, t3264: f64, t1671: f64, t3307: f64, t3265: f64, t4785: f64, t11190: f64, t3315: f64, t4781: f64, t3313: f64, t11277: f64, t1670: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15051 = t4782 * t1117;
    let t15053 = 4.0_f64 * t3264 * t15051;
    let t15054 = t1671 * t3307;
    let t15056 = 2.0_f64 * t3264 * t15054;
    let t15057 = t4785 * t3265;
    let t15059 = 0.96491876992155210402e2_f64 * t11190 * t15057;
    let t15060 = t4781 * t3315;
    let t15061 = t15060 * t1117;
    let t15063 = 0.32163958997385070134e2_f64 * t3313 * t15061;
    let t15064 = t4785 * t3307;
    let t15066 = 0.16081979498692535067e2_f64 * t3313 * t15064;
    let t15067 = t1670 * t11277;
    (t15051, t15053, t15054, t15056, t15057, t15059, t15060, t15061, t15063, t15064, t15066, t15067)
}
