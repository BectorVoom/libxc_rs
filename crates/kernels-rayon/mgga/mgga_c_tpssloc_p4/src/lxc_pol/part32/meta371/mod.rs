//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1424;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta371(t1227: f64, t15743: f64, t1725: f64, t698: f64, t1174: f64, t225: f64, t4941: f64, t5053: f64, t3701: f64, t5356: f64, t5168: f64, t592: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15745, t15753, t15754, t15797, t15820, t15868, t15877) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1424(t1227, t15743, t1725, t698, t1174, t225, t4941, t5053, t3701, t5356, t5168, t592);
    (t15745, t15753, t15754, t15797, t15820, t15868, t15877)
}
