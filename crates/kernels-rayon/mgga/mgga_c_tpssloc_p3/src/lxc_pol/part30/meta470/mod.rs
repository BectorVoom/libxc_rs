//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1758;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta470(t23476: f64, t343: f64, t23562: f64, t23384: f64, t6692: f64, t1049: f64, t6688: f64, t1054: f64, t1065: f64, t1921: f64, t2978: f64, t344: f64, t381: f64, t3034: f64, t38: f64, t131: f64, t350: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23563, t23564, t23579, t23581, t23587, t23588, t23592) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1758(t23476, t343, t23562, t23384, t6692, t1049, t6688, t1054, t1065, t1921, t2978, t344);
        let (t23593, t23598, t23599, t23600, t23601) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1759(t23592, t381, t3034, t38, t131, t350);
    (t23563, t23564, t23579, t23581, t23587, t23588, t23592, t23593, t23598, t23599, t23600, t23601)
}
