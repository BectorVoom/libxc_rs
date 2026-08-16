//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1031;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1032;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta272(t25: f64, t514: f64, t3665: f64, t606: f64, t3704: f64, t1298: f64, t2249: f64, t9257: f64, t28: f64, t517: f64, t1081: f64, t3673: f64, zeta_threshold: f64, t3711: f64, t11122: f64, t1302: f64, t3231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11985, t11987, t11988, t11991, t11997, t11998, t12000, t12001) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1031(t25, t514, t3665, t606, t3704, t1298, t2249, t9257, t28, t517, t1081, t3673, zeta_threshold);
        let (t12004, t12012) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1032(t28, t1081, t3711, t11122, t12000, t12001, t1302, t3231, t11997, zeta_threshold);
    (t11985, t11987, t11988, t11991, t11998, t12000, t12001, t12004, t12012)
}
