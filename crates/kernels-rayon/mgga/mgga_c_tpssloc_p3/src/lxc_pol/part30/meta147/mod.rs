//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta147 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk791;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk792;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta147(t3967: f64, t65: f64, t1410: f64, t628: f64, t1426: f64, t608: f64, t1409: f64, t2267: f64, t607: f64, t3966: f64, t43: f64, t2274: f64, t55: f64, t1414: f64, t1420: f64, t2282: f64, t39: f64, t51: f64, t615: f64, t621: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3968, t3971, t3976, t3981, t3982, t3985, t3990) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk791(t3967, t65, t1410, t628, t1426, t608, t1409, t2267, t607, t3966, t43, t2274);
        let (t3991, t3994, t3997) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk792(t3990, t607, t3966, t55, t1414, t1420, t2282, t39, t3982, t3985, t51, t615, t621);
    (t3968, t3971, t3976, t3981, t3990, t3991, t3994, t3997)
}
