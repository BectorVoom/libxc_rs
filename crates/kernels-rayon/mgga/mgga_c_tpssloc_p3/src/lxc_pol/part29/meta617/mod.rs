//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2058;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2059;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta617(t3545: f64, t7372: f64, t7378: f64, t24698: f64, t7327: f64, t2121: f64, t3427: f64, t7381: f64, t24574: f64, t24795: f64, t24799: f64, t3590: f64, t477: f64, t7365: f64, t85660: f64, t1170: f64, t24829: f64, t131: f64, t467: f64, t50: f64, t82510: f64, t10469: f64, t461: f64, t11721: f64, t3032: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t85917, t85918, t85920, t85941, t85943, t85945, t85947) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2058(t3545, t7372, t7378, t24698, t7327, t2121, t3427, t7381, t24574, t24795, t24799, t3590, t477);
        let (t85952, t85955, t85963, t85964, t85966) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2059(t7365, t85660, t1170, t2121, t24829, t131, t467, t50, t82510, t10469, t461, t11721, t3032);
    (t85917, t85918, t85920, t85941, t85943, t85945, t85947, t85952, t85955, t85963, t85964, t85966)
}
