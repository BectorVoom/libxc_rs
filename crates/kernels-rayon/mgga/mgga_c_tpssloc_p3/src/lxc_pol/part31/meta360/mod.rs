//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1278;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1279;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1280;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta360(t12189: f64, t1804: f64, t5194: f64, t782: f64, t5198: f64, t3732: f64, t67: f64, t792: f64, t1799: f64, t212: f64, t1307: f64, t686: f64, t12214: f64, t131: f64, t205: f64, t3726: f64, t5206: f64, t12199: f64, t5202: f64, t118: f64, t5187: f64, t794: f64, t3739: f64, t12225: f64, t2586: f64, t1338: f64, t5318: f64, t3866: f64, t5310: f64, t3799: f64, t5289: f64, t2371: f64, t5154: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16078, t16083, t16094, t16095, t16097) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1278(t12189, t1804, t5194, t782, t5198, t3732, t67, t792, t1799, t212, t1307, t686);
        let (t16099, t16101, t16106, t16108, t16111) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1279(t16094, t16097, t12214, t131, t205, t3726, t5206, t12199, t5202, t118, t5187, t794);
        let (t16113, t16119, t16132, t16147, t16159, t16164) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1280(t16111, t3739, t12225, t16095, t2586, t1338, t5318, t3866, t5310, t3799, t5289, t2371, t5154);
    (t16078, t16083, t16099, t16101, t16106, t16108, t16113, t16119, t16132, t16147, t16159, t16164)
}
