//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1137;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1138;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1139;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1140;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta267(t6976: f64, t7736: f64, t1992: f64, t1834: f64, t1998: f64, t214: f64, t1985: f64, t2031: f64, t7445: f64, t5: f64, t1860: f64, t2032: f64, t7026: f64, t7034: f64, t7428: f64, t7432: f64, t7435: f64, t112: f64, t1774: f64, t2039: f64, t109: f64, t7053: f64, t7464: f64, t510: f64, t1458: f64, t2075: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7737, t7738, t7740, t7741, t7742, t7782) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1137(t6976, t7736, t1992, t1834, t1998, t214, t1985, t2031, t7445);
        let (t7786, t7787, t7796) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1138(t5, t1860, t2032, t7026, t7034, t7428, t7432, t7435, t7782, t112, t1774, t2039);
        let t7801 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1139(t109, t7053, t7464);
        let t7802 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1140(t510, t7801);
        let t7806 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1141(t1458, t2075);
    (t7737, t7738, t7740, t7741, t7742, t7782, t7786, t7787, t7796, t7801, t7802, t7806)
}
