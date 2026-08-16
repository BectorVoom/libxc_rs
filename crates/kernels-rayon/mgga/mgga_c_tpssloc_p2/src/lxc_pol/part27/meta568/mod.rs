//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta568(t22674: f64, t22892: f64, t22916: f64, t22716: f64, t6908: f64, t22751: f64, t22930: f64, t22917: f64, t22723: f64, t22891: f64, t22920: f64, t117: f64, t5247: f64, t6559: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t80659, t80663, t80665, t80667, t80670, t80671, t80681) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2012(t22674, t22892, t22916, t22716, t6908, t22751, t22930, t22917, t22723, t22891, t22920, t117, t5247, t6559);
    (t80659, t80663, t80665, t80667, t80670, t80671, t80681)
}
