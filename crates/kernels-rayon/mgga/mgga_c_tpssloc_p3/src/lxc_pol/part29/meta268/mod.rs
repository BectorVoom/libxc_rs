//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta268 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1259;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1260;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta268(t6553: f64, t7488: f64, t1880: f64, t1492: f64, t1902: f64, t1496: f64, t6581: f64, t1484: f64, t236: f64, t1894: f64, t6591: f64, t1510: f64, t815: f64, t6605: f64, t1499: f64, t1898: f64, t249: f64, t1512: f64, t6614: f64, t1516: f64, t6621: f64, t6580: f64, t6587: f64, t6603: f64, t6618: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7489, t7490, t7492, t7494, t7496, t7497, t7498, t7500) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1259(t6553, t7488, t1880, t1492, t1902, t1496, t6581, t1484, t236, t1894, t6591, t1510, t815);
        let (t7503, t7510) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1260(t6605, t7500, t1499, t1898, t249, t1512, t6614, t1516, t6621, t6580, t6587, t6603, t6618, t7494, t7498);
    (t7489, t7490, t7492, t7496, t7497, t7500, t7503, t7510)
}
