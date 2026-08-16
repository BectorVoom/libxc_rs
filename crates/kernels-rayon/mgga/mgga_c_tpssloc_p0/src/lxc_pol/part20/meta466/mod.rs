//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1932;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta466(t11697: f64, t4949: f64, t3577: f64, t3431: f64, t4729: f64, t1174: f64, t1177: f64, t14749: f64, t14753: f64, t14744: f64, t1011: f64, t15031: f64, t1212: f64, t1226: f64, t4965: f64, t11652: f64, t11665: f64, t11678: f64, t11692: f64, t11699: f64, t11703: f64, t1218: f64, t1232: f64, t15560: f64, t15564: f64, t15569: f64, t3496: f64, t3580: f64, t4950: f64, t5002: f64) -> (f64, f64, f64, f64) {
        let (t15572, t15574, t15580, t15581, t15584, t15587, t15590) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1932(t11697, t4949, t3577, t3431, t4729, t1174, t1177, t14749, t14753, t14744, t1011, t15031);
        let (t15591, t15594, t15601) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1933(t1212, t15590, t1226, t4965, t11652, t11665, t11678, t11692, t11699, t11703, t1174, t1218, t1232, t15560, t15564, t15569, t15574, t15580, t15581, t15584, t15587, t3496, t3580, t4950, t5002);
    (t15572, t15591, t15594, t15601)
}
