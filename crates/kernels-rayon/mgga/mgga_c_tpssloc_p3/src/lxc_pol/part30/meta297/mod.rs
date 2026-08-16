//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta297 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1315;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta297(t2374: f64, t9888: f64, t2509: f64, t745: f64, t9843: f64, t761: f64, t152: f64, t31: f64, t2368: f64, t2505: f64, t746: f64, t9490: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9890, t9892, t9894, t9897, t9905, t9907, t9919) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1315(t2374, t9888, t2509, t745, t9843, t761, t152, t31, t2368, t2505, t746, t9490);
    (t9890, t9892, t9894, t9897, t9905, t9907, t9919)
}
