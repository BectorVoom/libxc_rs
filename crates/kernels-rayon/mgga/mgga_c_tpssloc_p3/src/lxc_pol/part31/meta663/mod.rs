//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1951;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1952;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta663(t1408: f64, t4255: f64, t870: f64, t25365: f64, t57911: f64, t10143: f64, t1484: f64, t25374: f64, t23788: f64, t67128: f64, t16949: f64, t25891: f64, t25927: f64, t98102: f64, t5966: f64, t868: f64, t1649: f64, t28248: f64, t83555: f64, t98030: f64, t98011: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99060, t100562, t100572, t100638, t100641) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1951(t1408, t4255, t870, t25365, t57911, t10143, t1484, t25374, t23788, t67128, t16949, t25891);
        let (t100644, t100646, t100651, t100656, t100659, t100664) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1952(t25927, t98102, t5966, t868, t1649, t4255, t870, t28248, t83555, t98030, t23788, t98011);
    (t99060, t100562, t100572, t100638, t100641, t100644, t100646, t100651, t100656, t100659, t100664)
}
