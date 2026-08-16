//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1366;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1367;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta365(t13716: f64, t942: f64, t951: f64, t959: f64, t2940: f64, t4489: f64, t10523: f64, t1580: f64, t2933: f64, t1543: f64, t2791: f64, t2794: f64, t4498: f64, t2925: f64, t4488: f64, t1634: f64, t3175: f64, t10165: f64, t1065: f64, t4693: f64, t3174: f64, t2970: f64, t4343: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13720, t13722, t13726, t13729) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1366(t13716, t942, t951, t959, t2940, t4489, t10523, t1580, t2933, t1543, t2791, t2794);
        let (t13731, t13734, t13736, t13743, t13748) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1367(t2940, t4498, t2925, t4488, t959, t1634, t3175, t10165, t1065, t4693, t3174, t2970, t4343);
    (t13720, t13722, t13726, t13729, t13731, t13734, t13736, t13743, t13748)
}
