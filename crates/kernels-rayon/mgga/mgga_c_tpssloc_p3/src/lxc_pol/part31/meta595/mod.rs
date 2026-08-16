//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1840;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta595(t12571: f64, t23966: f64, t6492: f64, t7432: f64, t84195: f64, t23967: f64, t26067: f64, t23993: f64, t7428: f64, t23998: f64, t1860: f64, t23992: f64, t7445: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t91957, t91959, t91961, t91980, t91996, t92001, t92003) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1840(t12571, t23966, t6492, t7432, t84195, t23967, t26067, t23993, t7428, t23998, t1860, t23992, t7445);
    (t91957, t91959, t91961, t91980, t91996, t92001, t92003)
}
