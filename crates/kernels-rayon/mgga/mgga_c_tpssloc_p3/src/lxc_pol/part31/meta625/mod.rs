//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta625(t19951: f64, t22833: f64, t19972: f64, t19976: f64, t5259: f64, t91100: f64, t26308: f64, t5234: f64, t5252: f64, t6396: f64, t80820: f64, t19962: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t97208, t97210, t97212, t97214, t97217, t97219, t97221) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1882(t19951, t22833, t19972, t19976, t5259, t91100, t26308, t5234, t5252, t6396, t80820, t19962);
    (t97208, t97210, t97212, t97214, t97217, t97219, t97221)
}
