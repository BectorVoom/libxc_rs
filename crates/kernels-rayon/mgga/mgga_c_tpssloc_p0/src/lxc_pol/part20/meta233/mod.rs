//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1326;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1327;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1328;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta233(t205: f64, t9558: f64, t210: f64, t214: f64, t9458: f64, t213: f64, t776: f64, t221: f64, t2553: f64, t59: f64, t8705: f64, t207: f64, t215: f64, t2570: f64, t782: f64, t2573: f64, t2690: f64, t154: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9559, t9561, t9566, t9569) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1326(t205, t9558, t210, t214, t9458, t213, t776, t221, t2553, t59, t8705);
        let (t9572, t9573) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1327(t207, t215, t9569, t2570, t782);
        let (t9574, t9576, t9577) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1328(t2573, t9573, t2690, t59, t154);
    (t9559, t9561, t9566, t9569, t9572, t9573, t9574, t9576, t9577)
}
