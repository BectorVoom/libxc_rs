//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta104 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk720;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk721;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk722;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta104(t116: f64, t206: f64, t212: f64, t2586: f64, t2562: f64, t2564: f64, t2569: f64, t2571: f64, t2573: f64, t2579: f64, t2582: f64, t787: f64, t252: f64, t798: f64, t852: f64, t225: f64, t799: f64, t154: f64, t2559: f64, t222: f64, t2563: f64, t805: f64, t119: f64, t2379: f64, t210: f64, t2553: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2588, t2590, t2591) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk720(t116, t206, t212, t2586, t2562, t2564, t2569, t2571, t2573, t2579, t2582, t787);
        let (t2592, t2594, t2597) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk721(t252, t2591, t798, t852, t225, t799);
        let (t2600, t2602, t2603, t2606, t2610, t2613) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk722(t154, t2559, t222, t2563, t805, t119, t2379, t210, t2553, t225, t2591);
    (t2588, t2590, t2591, t2592, t2594, t2597, t2600, t2602, t2603, t2606, t2610, t2613)
}
