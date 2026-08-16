//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta119 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk608;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk609;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta119(t457: f64, t974: f64, t1721: f64, t225: f64, t1222: f64, t1731: f64, t1744: f64, t1229: f64, t3247: f64, t3242: f64, t3584: f64, t1653: f64, t248: f64, t3521: f64, t1227: f64, t1735: f64, t3570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4934, t4945, t4957, t4959, t4972, t4987, t4993) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk608(t457, t974, t1721, t225, t1222, t1731, t1744, t1229, t3247, t3242, t3584, t1653, t248, t3521);
        let (t4994, t4997) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk609(t1227, t4993, t1735, t248, t3570);
    (t4934, t4945, t4957, t4959, t4972, t4987, t4993, t4994, t4997)
}
