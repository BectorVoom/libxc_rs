//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1272;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1273;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta356(t1213: f64, t15730: f64, t11789: f64, t1653: f64, t248: f64, t1227: f64, t15437: f64, t3505: f64, t3576: f64, t5064: f64, t13969: f64, t4988: f64, t1725: f64, t698: f64, t1174: f64, t225: f64, t4941: f64, t5053: f64, t3701: f64, t5356: f64, t5168: f64, t592: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15731, t15735, t15737, t15740, t15743) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1272(t1213, t15730, t11789, t1653, t248, t1227, t15437, t3505, t3576, t5064, t13969, t4988);
        let (t15745, t15754, t15797, t15820, t15868, t15877) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1273(t1227, t15743, t1725, t698, t1174, t225, t4941, t5053, t3701, t5356, t5168, t592);
    (t15731, t15735, t15737, t15740, t15745, t15754, t15797, t15820, t15868, t15877)
}
