//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2071;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta602(t1920: f64, t23353: f64, t968: f64, t10164: f64, t225: f64, t23384: f64, t23595: f64, t23408: f64, t1921: f64, t6733: f64, t3034: f64, t336: f64, t131: f64, t350: f64, t38: f64, t10469: f64, t344: f64, t10482: f64, t3032: f64, t2261: f64, t6794: f64, t23598: f64, t614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82463, t82481, t82490, t82499, t82502, t82510) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2071(t1920, t23353, t968, t10164, t225, t23384, t23595, t23408, t1921, t6733, t3034, t336);
        let (t82513, t82514, t82516, t82527, t82534) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2072(t131, t350, t38, t82510, t10469, t344, t10482, t3032, t2261, t6794, t23598, t614);
    (t82463, t82481, t82490, t82499, t82502, t82513, t82514, t82516, t82527, t82534)
}
