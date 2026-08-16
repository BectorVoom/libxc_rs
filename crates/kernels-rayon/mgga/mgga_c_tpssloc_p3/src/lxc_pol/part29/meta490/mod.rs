//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1838;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta490(t24826: f64, t7378: f64, t2147: f64, t3590: f64, t462: f64, t7319: f64, t7327: f64, t7377: f64, t2144: f64, t3507: f64, t3625: f64, t1215: f64, t7348: f64, t1246: f64, t1170: f64, t7381: f64, t2121: f64, t210: f64, t7371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24827, t24829, t24830, t24833) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1838(t24826, t7378, t2147, t3590, t462, t7319, t7327);
        let (t24834, t24837, t24838, t24841, t24844, t24845, t24847) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1839(t24833, t7377, t2144, t3507, t3625, t1215, t7348, t1246, t1170, t7381, t2121, t210, t7371);
    (t24827, t24829, t24830, t24833, t24834, t24837, t24838, t24841, t24844, t24845, t24847)
}
