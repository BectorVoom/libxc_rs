//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2018;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2019;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta458(t25: f64, t12061: f64, t1408: f64, t2: f64, t3664: f64, t584: f64, t606: f64, t16: f64, t2249: f64, t3665: f64, t5134: f64, t5137: f64, t514: f64, zeta_threshold: f64, t28: f64, t12072: f64, t1649: f64, t3672: f64, t1081: f64, t3231: f64, t3673: f64, t5142: f64, t5145: f64, t517: f64, t157: f64) -> (f64, f64, f64, f64, f64) {
        let (t15937, t15941, t15951) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2018(t25, t12061, t1408, t2, t3664, t584, t606, t16, t2249, t3665, t5134, t5137, t514, zeta_threshold);
        let (t15952, t15956, t15966) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2019(t28, t12072, t1649, t2, t3672, t1081, t584, t16, t3231, t3673, t5142, t5145, t517, zeta_threshold);
        let t15968 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2020(t157, t15951, t15966);
    (t15937, t15941, t15952, t15956, t15968)
}
