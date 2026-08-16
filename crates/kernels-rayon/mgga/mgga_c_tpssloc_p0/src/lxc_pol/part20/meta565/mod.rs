//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2124;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta565(t10401: f64, t10935: f64, t3186: f64, t3200: f64, t11051: f64, t3069: f64, t10454: f64, t3048: f64, t10459: f64, t3036: f64, t3087: f64, t3033: f64, t3128: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t42505, t42508, t42511, t42514, t42518, t42520, t42522) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2124(t10401, t10935, t3186, t3200, t11051, t3069, t10454, t3048, t10459, t3036, t3087, t3033, t3128);
    (t42505, t42508, t42511, t42514, t42518, t42520, t42522)
}
