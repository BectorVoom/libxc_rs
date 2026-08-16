//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta512(t343: f64, t381: f64, t6690: f64, t25712: f64, t4347: f64, t6689: f64, t7561: f64, t968: f64, t1920: f64, t1625: f64, t6688: f64, t6691: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25796, t25797, t25798, t25801, t25802, t25806, t25807, t25810, t25811) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1836(t343, t381, t6690, t25712, t4347, t6689, t7561, t968, t1920, t1625, t6688, t6691);
    (t25796, t25797, t25798, t25801, t25802, t25806, t25807, t25810, t25811)
}
