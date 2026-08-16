//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta113 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk619;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk620;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta113(t3166: f64, t349: f64, t1050: f64, t225: f64, t1053: f64, t386: f64, t68: f64, t1065: f64, t1057: f64, t3112: f64, t3032: f64, t3127: f64, t3031: f64, t3040: f64, t381: f64, t1932: f64, t3131: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3167, t3169, t3174, t3175, t3176, t3180) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk619(t3166, t349, t1050, t225, t1053, t386, t68, t1065, t1057, t3112);
        let (t3185, t3186) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk620(t3032, t3127, t3031);
        let (t3187, t3188) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk621(t3040, t381, t1932, t3131);
    (t3167, t3169, t3174, t3175, t3176, t3180, t3185, t3186, t3187, t3188)
}
