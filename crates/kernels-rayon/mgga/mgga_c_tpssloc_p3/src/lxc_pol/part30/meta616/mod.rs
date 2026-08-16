//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta616(t25721: f64, t6743: f64, t210: f64, t23599: f64, t23632: f64, t23511: f64, t23634: f64, t23518: f64, t6692: f64, t82632: f64, t23357: f64, t6680: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t83240, t83244, t83245, t83246, t83265, t83281, t83344) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2015(t25721, t6743, t210, t23599, t23632, t23511, t23634, t23518, t6692, t82632, t23357, t6680);
    (t83240, t83244, t83245, t83246, t83265, t83281, t83344)
}
