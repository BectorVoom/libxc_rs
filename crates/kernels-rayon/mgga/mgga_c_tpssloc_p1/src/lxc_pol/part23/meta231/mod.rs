//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta231 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk881;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta231(t5519: f64, t706: f64, t13115: f64, t157: f64, t5398: f64, t751: f64, t707: f64, t5522: f64, t67: f64, t758: f64, t184: f64, t5392: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16689, t16693, t16701, t16702, t16710, t16711, t16716) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk881(t5519, t706, t13115, t157, t5398, t751, t707, t5522, t67, t758, t184, t5392);
    (t16689, t16693, t16701, t16702, t16710, t16711, t16716)
}
