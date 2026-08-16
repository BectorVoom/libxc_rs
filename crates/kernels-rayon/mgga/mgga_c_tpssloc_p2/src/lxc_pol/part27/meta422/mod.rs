//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta422(t532: f64, t6995: f64, t6879: f64, t1983: f64, t2018: f64, t531: f64, t1390: f64, t3734: f64, t1868: f64, t2319: f64, t6876: f64, t6997: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22591, t22592, t22594, t22596, t22597, t22599, t22600, t22605) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1731(t532, t6995, t6879, t1983, t2018, t531, t1390, t3734, t1868, t2319, t6876, t6997);
    (t22591, t22592, t22594, t22596, t22597, t22599, t22600, t22605)
}
