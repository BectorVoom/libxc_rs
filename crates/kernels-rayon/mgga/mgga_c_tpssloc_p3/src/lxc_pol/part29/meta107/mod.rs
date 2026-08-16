//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta107 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk675;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta107(t40: f64, t52: f64, t2427: f64, t708: f64, t607: f64, t751: f64, t707: f64, t195: f64, t2244: f64, t2250: f64, t73: f64, t197: f64, t76: f64, t157: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2429, t2430, t2431, t2432, t2433, t2440, t2447, t2448) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk675(t40, t52, t2427, t708, t607, t751, t707, t195, t2244, t2250, t73, t197, t76, t157, zeta_threshold);
    (t2429, t2430, t2431, t2432, t2433, t2440, t2447, t2448)
}
