//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1880;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1881;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1882;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1883;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta539(t7359: f64, t7999: f64, t1186: f64, t8077: f64, t1222: f64, t8043: f64, t6729: f64, t8027: f64, t2140: f64, t4965: f64, t1202: f64, t8048: f64, t8049: f64, t5017: f64, t7337: f64, t1207: f64, t1218: f64, t2136: f64, t24675: f64, t24681: f64, t24690: f64, t24704: f64, t488: f64, t4974: f64, t5014: f64, t5030: f64, t7339: f64, t7345: f64, t2139: f64, t5022: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27572, t27574, t27578, t27580, t27586, t27589) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1880(t7359, t7999, t1186, t8077, t1222, t8043, t6729, t8027, t2140, t4965, t1202, t8048);
        let (t27592, t27598, t27599) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1881(t1222, t8049, t5017, t7337, t1207);
        let t27602 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1882(t1218, t2136, t24675, t24681, t24690, t24704, t27578, t27580, t27586, t27589, t27592, t27599, t488, t4974, t5014, t5030, t7339, t7345);
        let (t27603, t27604) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1883(t2139, t5022, t471);
    (t27572, t27574, t27578, t27580, t27586, t27589, t27592, t27598, t27599, t27602, t27603, t27604)
}
