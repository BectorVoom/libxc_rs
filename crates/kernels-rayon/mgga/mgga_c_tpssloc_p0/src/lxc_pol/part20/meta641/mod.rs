//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta641 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2348;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2349;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta641(t42444: f64, t45971: f64, t48140: f64, t2770: f64, t340: f64, t43317: f64, t136: f64, t47746: f64, t908: f64, t2403: f64, t4389: f64, t4386: f64, t13543: f64, t699: f64, t13547: f64, t13556: f64, t13529: f64, t13533: f64, t41887: f64, t41889: f64, t43002: f64, t48122: f64, t48125: f64, t48128: f64, t48131: f64, t48134: f64, t48137: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48142, t48145, t48148, t48153, t48155, t48156, t48157) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2348(t42444, t45971, t48140, t2770, t340, t43317, t136, t47746, t908, t2403, t4389, t4386);
        let (t48159, t48161, t48163, t48165, t48167, t48169) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2349(t48157, t13543, t699, t13547, t13556, t13529, t13533, t41887, t41889, t43002, t48122, t48125, t48128, t48131, t48134, t48137, t48142, t48145, t48148, t48153, t48156);
    (t48142, t48145, t48148, t48153, t48155, t48157, t48159, t48161, t48163, t48165, t48167, t48169)
}
