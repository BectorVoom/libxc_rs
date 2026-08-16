//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1310;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1311;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta286(t180: f64, t2511: f64, t9489: f64, t9490: f64, t761: f64, t116: f64, t229: f64, t212: f64, t776: f64, t2586: f64, t597: f64, t60: f64, t59: f64, t2386: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9493, t9494, t9496, t9523, t9526, t9533) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1310(t180, t2511, t9489, t9490, t761, t116, t229, t212, t776, t2586, t597, t60);
        let (t9534, t9537) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1311(t59, t9533, t212, t2386);
    (t9493, t9494, t9496, t9523, t9526, t9533, t9534, t9537)
}
