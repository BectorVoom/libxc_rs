//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1483;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta310(t225: f64, t4943: f64, t1720: f64, t3030: f64, t3609: f64, t1009: f64, t4940: f64, t1243: f64, t11277: f64, t1670: f64, t14704: f64, t14710: f64, t14720: f64, t14781: f64, t1147: f64, t4832: f64, t1687: f64, t3400: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14980, t15026, t15027) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1483(t225, t4943, t1720, t3030, t3609);
        let (t15031, t15032, t15067, t15072, t15074, t15083, t15094, t15121, t15126) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1484(t1009, t4940, t1243, t11277, t1670, t14704, t14710, t14720, t14781, t1147, t4832, t1687, t3400);
    (t14980, t15026, t15027, t15031, t15032, t15067, t15072, t15074, t15083, t15094, t15121, t15126)
}
