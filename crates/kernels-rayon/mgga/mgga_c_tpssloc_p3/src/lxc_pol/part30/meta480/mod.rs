//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1777;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1778;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta480(t25084: f64, t4184: f64, t23146: f64, t4191: f64, t4240: f64, t4250: f64, t13228: f64, t828: f64, t2628: f64, t6605: f64, t13351: f64, t232: f64, t815: f64, t23097: f64, t23096: f64, t23106: f64, t23108: f64, t23114: f64, t23119: f64, t1894: f64, t236: f64, t4119: f64, t6591: f64, t23062: f64, t7497: f64, t1510: f64, t776: f64, t13223: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25085, t25087, t25089, t25091, t25093, t25094, t25095, t25097) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1777(t25084, t4184, t23146, t4191, t4240, t4250, t13228, t828, t2628, t6605, t13351, t232);
        let (t25098, t25103) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1778(t25097, t815, t23097, t23096, t23106, t23108, t23114, t23119, t25085, t25087, t25089, t25091, t25095);
        let (t25106, t25107, t25109, t25111, t25112, t25113, t25115) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1779(t1894, t236, t4119, t6591, t23062, t7497, t1510, t776, t815, t23097, t13223, t232);
    (t25093, t25094, t25097, t25098, t25103, t25106, t25107, t25109, t25111, t25112, t25113, t25115)
}
