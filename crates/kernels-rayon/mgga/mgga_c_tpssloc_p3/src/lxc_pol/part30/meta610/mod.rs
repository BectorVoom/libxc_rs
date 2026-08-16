//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2004;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2005;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta610(t10336: f64, t1920: f64, t1922: f64, t1049: f64, t23592: f64, t10164: f64, t225: f64, t1921: f64, t6733: f64, t3034: f64, t336: f64, t131: f64, t350: f64, t38: f64, t10469: f64, t344: f64, t10482: f64, t3032: f64, t23598: f64, t614: f64, t3131: f64, t23383: f64, t6712: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82436, t82469, t82481, t82502, t82513) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2004(t10336, t1920, t1922, t1049, t23592, t10164, t225, t1921, t6733, t3034, t336, t131, t350, t38);
        let (t82514, t82516, t82534, t82542, t82573) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2005(t10469, t344, t10482, t3032, t131, t23598, t350, t614, t3131, t23383, t6712);
    (t82436, t82469, t82481, t82502, t82513, t82514, t82516, t82534, t82542, t82573)
}
