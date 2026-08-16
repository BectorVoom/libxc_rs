//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1269;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1270;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta432(t19033: f64, t4993: f64, t19046: f64, t5018: f64, t5023: f64, t6169: f64, t18321: f64, t5040: f64, t1009: f64, t22113: f64, t1011: f64, t1212: f64, t18375: f64, t5002: f64, t1730: f64, t19032: f64, t1017: f64, t1207: f64, t1210: f64, t22173: f64, t372: f64, t471: f64, t479: f64, t15507: f64, t19095: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t72302, t72304, t72307, t72352, t72361, t72363) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1269(t19033, t4993, t19046, t5018, t5023, t6169, t18321, t5040, t1009, t22113, t1011, t1212);
        let (t72366, t72384, t72389, t72398, t72403) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1270(t18375, t5002, t1730, t19032, t1017, t1207, t1210, t22173, t372, t471, t479, t15507, t19095);
    (t72302, t72304, t72307, t72352, t72361, t72363, t72366, t72384, t72389, t72398, t72403)
}
