//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1823;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1824;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta505(t343: f64, t4540: f64, t6734: f64, t4571: f64, t6765: f64, t4630: f64, t6755: f64, t1611: f64, t6758: f64, t1036: f64, t7586: f64, t1409: f64, t1933: f64, t1937: f64, t1618: f64, t1622: f64, t1935: f64, t23433: f64, t23443: f64, t23447: f64, t23449: f64, t23463: f64, t23469: f64, t23529: f64, t378: f64, t6730: f64, t7578: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25608, t25609, t25616, t25618, t25622, t25625, t25628) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1823(t343, t4540, t6734, t4571, t6765, t4630, t6755, t1611, t6758, t1036, t7586, t1409, t1933);
        let (t25629, t25631) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1824(t1937, t25628, t1618, t1622, t1935, t23433, t23443, t23447, t23449, t23463, t23469, t23529, t25609, t25616, t25618, t25622, t25625, t378, t6730, t7578);
    (t25608, t25609, t25616, t25618, t25622, t25625, t25629, t25631)
}
