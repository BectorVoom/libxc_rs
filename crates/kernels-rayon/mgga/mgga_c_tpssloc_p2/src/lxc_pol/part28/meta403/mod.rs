//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1562;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1563;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta403(t22473: f64, t2332: f64, t2358: f64, t6530: f64, t2303: f64, t71: f64, t33: f64, t9228: f64, t2235: f64, t608: f64, t641: f64, t645: f64, t72: f64, t2307: f64, t79: f64, t2244: f64, t605: f64, t2251: f64, t2241: f64, t2240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22474, t22476, t22489, t22493, t22519, t22527) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1562(t22473, t2332, t2358, t6530, t2303, t71, t33, t9228, t2235, t608, t641, t645, t72);
        let (t22531, t22534, t22537, t22546, t22549) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1563(t2307, t79, t72, t2244, t605, t2251, t2241, t2240, t608);
    (t22474, t22476, t22489, t22493, t22519, t22527, t22531, t22534, t22537, t22546, t22549)
}
