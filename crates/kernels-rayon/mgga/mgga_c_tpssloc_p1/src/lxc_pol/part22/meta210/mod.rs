//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta210 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1210;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta210(t479: f64, t6163: f64, t471: f64, t225: f64, t6150: f64) -> (f64, f64, f64) {
        let (t6164, t6165, t6168) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1210(t479, t6163, t471, t225, t6150);
    (t6164, t6165, t6168)
}
