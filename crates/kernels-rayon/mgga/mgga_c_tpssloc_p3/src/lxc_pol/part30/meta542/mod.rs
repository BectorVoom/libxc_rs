//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta542(t25962: f64, t25999: f64, t26155: f64, t26507: f64, t3: f64, t112: f64, t7758: f64, t16521: f64, t1873: f64, t16524: f64, t7015: f64, t5371: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t26509, t26510, t26523, t26533, t26535, t26537) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1892(t25962, t25999, t26155, t26507, t3, t112, t7758, t16521, t1873, t16524, t7015, t5371, t6534);
    (t26509, t26510, t26523, t26533, t26535, t26537)
}
