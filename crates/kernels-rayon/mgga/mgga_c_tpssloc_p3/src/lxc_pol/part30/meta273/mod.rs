//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1237;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1238;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1239;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta273(t1894: f64, t7496: f64, t6591: f64, t1510: f64, t815: f64, t6605: f64, t1499: f64, t1898: f64, t249: f64, t1512: f64, t6614: f64, t1516: f64, t6621: f64, t6580: f64, t6587: f64, t6603: f64, t6618: f64, t7494: f64, t218: f64, t1527: f64, t1911: f64, t2718: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t7497, t7498, t7500, t7501, t7503, t7504, t7506, t7508) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1237(t1894, t7496, t6591, t1510, t815, t6605, t1499, t1898, t249, t1512, t6614, t1516, t6621);
        let t7510 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1238(t6580, t6587, t6603, t6618, t7494, t7498, t7501, t7504, t7506, t7508);
        let (t7511, t7517) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1239(t218, t7510, t1527, t1911, t2718);
    (t7497, t7500, t7503, t7510, t7511, t7517)
}
