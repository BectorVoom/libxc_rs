//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1935;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta468(t15621: f64, t4582: f64, t11721: f64, t3507: f64, t4977: f64, t3509: f64, t1216: f64, t15553: f64, t13969: f64, t4979: f64, t3506: f64, t4973: f64, t1227: f64, t11705: f64, t11719: f64, t11728: f64, t11734: f64, t11746: f64, t15610: f64, t15612: f64, t15617: f64, t3490: f64, t3496: f64, t3515: f64, t4974: f64, t4984: f64, t5019: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15622, t15625, t15626, t15627, t15630, t15631, t15636, t15637, t15640, t15642, t15643) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1935(t15621, t4582, t11721, t3507, t4977, t3509, t1216, t15553, t13969, t4979, t3506, t4973);
        let t15648 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1936(t1227, t15643, t11705, t11719, t11728, t11734, t11746, t15610, t15612, t15617, t15622, t15627, t15631, t15637, t15642, t3490, t3496, t3506, t3515, t4974, t4984, t5019);
    (t15622, t15625, t15626, t15627, t15630, t15631, t15636, t15637, t15640, t15643, t15648)
}
