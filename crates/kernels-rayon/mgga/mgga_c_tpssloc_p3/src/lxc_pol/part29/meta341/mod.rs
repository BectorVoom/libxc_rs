//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1401;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1402;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta341(t28: f64, t528: f64, t1294: f64, t9722: f64, t172: f64, t3681: f64, t763: f64, t2528: f64, t3691: f64, t9919: f64, t2663: f64, t3814: f64, t67: f64, t758: f64, t9905: f64, t9892: f64, t3684: f64, t9467: f64, t118: f64, t1284: f64, t2375: f64, t9882: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12072, t12087, t12089, t12091, t12094, t12097) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1401(t28, t528, t1294, t9722, t172, t3681, t763, t2528, t3691, t9919, t2663, t3814);
        let (t12100, t12103, t12105, t12109, t12111, t12114) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1402(t3681, t67, t758, t1294, t9905, t9892, t3684, t9467, t118, t1284, t2375, t9882);
    (t12072, t12087, t12089, t12091, t12094, t12097, t12100, t12103, t12105, t12109, t12111, t12114)
}
