//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1963;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1964;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1965;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta481(t25: f64, t1788: f64, t2225: f64, t2221: f64, t2223: f64, t12130: f64, t11987: f64, t1408: f64, t2: f64, t3704: f64, t1298: f64, t15941: f64, t16: f64, t2249: f64, t3665: f64, t5170: f64, t5173: f64, t584: f64, zeta_threshold: f64, t28: f64, t12000: f64, t1649: f64, t3711: f64, t1302: f64, t15956: f64, t3231: f64, t3673: f64, t5178: f64, t5181: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15983, t15985, t15987, t15988, t15989, t15992, t16002) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1963(t25, t1788, t2225, t2221, t2223, t12130, t11987, t1408, t2, t3704, t1298, t15941, t16, t2249, t3665, t5170, t5173, t584, zeta_threshold);
        let (t16003, t16006, t16016) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1964(t28, t12000, t1649, t2, t3711, t1302, t15956, t16, t3231, t3673, t5178, t5181, t584, zeta_threshold);
        let t16018 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1965(t16002, t16016);
    (t15983, t15985, t15987, t15988, t15989, t15992, t16003, t16006, t16018)
}
