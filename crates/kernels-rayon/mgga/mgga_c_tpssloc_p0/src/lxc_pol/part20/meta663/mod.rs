//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta663(t14725: f64, t9288: f64, t136: f64, t3297: f64, t14748: f64, t2250: f64, t1113: f64, t14735: f64, t2244: f64, t4728: f64, t9258: f64, t43768: f64, t43770: f64, t43777: f64, t50846: f64, t50848: f64, t50851: f64, t50854: f64, t50859: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50861, t50863, t50865, t50867, t50869, t50871, t50873, t50875, t50877) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2484(t14725, t9288, t136, t3297, t14748, t2250, t1113, t14735, t2244, t4728, t9258, t43768, t43770, t43777, t50846, t50848, t50851, t50854, t50859);
    (t50861, t50863, t50865, t50867, t50869, t50871, t50873, t50875, t50877)
}
