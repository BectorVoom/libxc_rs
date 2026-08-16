//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta126 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk726;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk727;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk728;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk729;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta126(t1932: f64, t3131: f64, t1014: f64, t3032: f64, t3031: f64, t360: f64, t390: f64, t1878: f64, t268: f64, t405: f64, t1091: f64, t690: f64, t1229: f64, t154: f64, t636: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3188, t3199, t3200) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk726(t1932, t3131, t1014, t3032, t3031);
        let (t3201, t3215, t3216, t3236, t3237, t3238) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk727(t1932, t360, t390, t1878, t268, t405, t1091, t690);
        let t3240 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk728(t1229, t154);
        let (t3241, t3242) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk729(t636);
    (t3188, t3199, t3200, t3201, t3215, t3216, t3236, t3237, t3238, t3240, t3241, t3242)
}
