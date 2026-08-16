//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta170 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk863;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta170(t1041: f64, t4571: f64, t1616: f64, t884: f64, t3071: f64, t1023: f64, t1539: f64, t247: f64, t375: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t4572, t4574, t4575, t4578, t4579, t4582) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk863(t1041, t4571, t1616, t884, t3071, t1023, t1539, t247, t375);
    (t4572, t4574, t4575, t4578, t4579, t4582)
}
