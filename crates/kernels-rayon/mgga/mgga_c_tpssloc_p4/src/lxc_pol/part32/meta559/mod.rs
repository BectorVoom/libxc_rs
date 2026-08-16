//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1923;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta559(t6420: f64, t6987: f64, t1825: f64, t26458: f64, t19743: f64, t550: f64, t6976: f64, t1992: f64, t1336: f64, t22718: f64, t22726: f64, t26437: f64, t27096: f64, t28156: f64, t28161: f64, t28165: f64, t28169: f64, t28171: f64, t5234: f64, t544: f64, t7745: f64, t28155: f64, t1378: f64, t1375: f64, t1843: f64, t20029: f64, t20044: f64, t2016: f64, t22646: f64, t26184: f64, t26345: f64, t26477: f64, t26988: f64, t26993: f64, t28051: f64, t28053: f64, t28108: f64, t28111: f64, t28118: f64, t5215: f64, t568: f64, t6461: f64, t6958: f64, t7729: f64, t7750: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t28174, t28178, t28181, t28182, t28185) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1923(t6420, t6987, t1825, t26458, t19743, t550, t6976, t1992, t1336, t22718, t22726, t26437, t27096, t28156, t28161, t28165, t28169, t28171, t5234, t544, t7745);
        let (t28186, t28187, t28190) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1924(t28155, t28185, t1378, t1375, t1843, t20029, t20044, t2016, t22646, t26184, t26345, t26477, t26988, t26993, t28051, t28053, t28108, t28111, t28118, t5215, t568, t6461, t6958, t7729, t7750);
    (t28174, t28178, t28181, t28182, t28186, t28187, t28190)
}
