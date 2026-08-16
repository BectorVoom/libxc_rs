//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1911;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1912;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta605(t1985: f64, t7700: f64, t80707: f64, t214: f64, t5318: f64, t6888: f64, t6891: f64, t16065: f64, t1992: f64, t22897: f64, t26378: f64, t6914: f64, t16044: f64, t6976: f64, t1372: f64, t1799: f64, t1307: f64, t26331: f64, t26446: f64, t26411: f64, t12420: f64, t5335: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90737, t90739, t90741, t90747, t90749) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1911(t1985, t7700, t80707, t214, t5318, t6888, t6891, t16065, t1992, t22897, t26378, t6914);
        let (t90752, t90754, t90757, t90759, t90763) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1912(t16044, t1992, t6976, t1372, t1799, t1307, t26331, t26446, t26411, t6914, t12420, t5335);
    (t90737, t90739, t90741, t90747, t90749, t90752, t90754, t90757, t90759, t90763)
}
