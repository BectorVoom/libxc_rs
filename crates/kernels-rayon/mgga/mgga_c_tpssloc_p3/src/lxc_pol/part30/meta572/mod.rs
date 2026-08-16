//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1942;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1943;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta572(t28593: f64, t383: f64, t1058: f64, t1920: f64, t23619: f64, t25465: f64, t25508: f64, t28597: f64, t28602: f64, t28605: f64, t28610: f64, t28614: f64, t28618: f64, t28622: f64, t28626: f64, t28631: f64, t3200: f64, t353: f64, t4669: f64, t6687: f64, t6797: f64, t7620: f64, t5677: f64, t6785: f64, t23696: f64, t1945: f64, t5866: f64, t1060: f64, t25470: f64, t7603: f64, t1409: f64, t1615: f64, t6800: f64, t23635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28634, t28636) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1942(t28593, t383, t1058, t1920, t23619, t25465, t25508, t28597, t28602, t28605, t28610, t28614, t28618, t28622, t28626, t28631, t3200, t353, t4669, t6687, t6797, t7620);
        let (t28637, t28638, t28641, t28642, t28648, t28651, t28652, t28653) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1943(t5677, t6785, t23696, t1945, t5866, t1060, t25470, t7603, t1409, t1615, t6800, t23635);
    (t28634, t28636, t28637, t28638, t28641, t28642, t28648, t28651, t28652, t28653)
}
