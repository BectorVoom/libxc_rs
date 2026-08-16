//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1263;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1264;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1265;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1266;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1267;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1268;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta280(t2109: f64, t7445: f64, t5: f64, t1860: f64, t2110: f64, t7246: f64, t7428: f64, t7432: f64, t7435: f64, t7975: f64, t112: f64, t25: f64, t265: f64, t394: f64, t1458: f64, t2165: f64, t7642: f64, t1409: f64, t2116: f64, t40: f64, t7552: f64, t1419: f64, t337: f64, t1887: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1653: f64, t7286: f64, t7285: f64, t1716: f64, t2123: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7978 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1263(t2109, t7445);
        let (t7982, t7983) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1264(t5, t1860, t2110, t7246, t7428, t7432, t7435, t7975, t7978, t112);
        let (t7989, t7992, t7997, t7998, t7999) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1265(t25, t265, t394, t1458, t2165, t7642, t1409, t2116, t40, t7552, t1419, t337, t1887, dens_threshold, rho0, zeta_threshold);
        let t8002 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1266(t1653, t7286);
        let t8003 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1267(t7285, t8002);
        let t8006 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1268(t1716, t2123);
    (t7978, t7982, t7983, t7989, t7992, t7997, t7998, t7999, t8002, t8003, t8006)
}
