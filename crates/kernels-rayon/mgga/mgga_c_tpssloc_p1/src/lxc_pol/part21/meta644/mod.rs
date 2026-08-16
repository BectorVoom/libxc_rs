//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta644 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2435;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2436;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta644(t10970: f64, t820: f64, t1041: f64, t10868: f64, t248: f64, t2780: f64, t10277: f64, t976: f64, t11046: f64, t42387: f64, t10457: f64, t10936: f64, t3180: f64, t10401: f64, t10935: f64, t3186: f64, t3200: f64, t11051: f64, t3069: f64, t3036: f64, t3087: f64, t3033: f64, t3128: f64, t10402: f64, t11034: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42397, t42432, t42444, t42483, t42488, t42496) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2435(t10970, t820, t1041, t10868, t248, t2780, t10277, t976, t11046, t42387, t10457, t10936, t3180);
        let (t42505, t42508, t42511, t42520, t42522, t42541) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2436(t10401, t10935, t3186, t3200, t11051, t3069, t3036, t3087, t3033, t3128, t10402, t11034);
    (t42397, t42432, t42444, t42483, t42488, t42496, t42505, t42508, t42511, t42520, t42522, t42541)
}
