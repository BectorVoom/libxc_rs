//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta750 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta750(t12339: f64, t5314: f64, t1831: f64, t40059: f64, t16336: f64, t3872: f64, t16060: f64, t3865: f64, t1369: f64, t16123: f64, t68: f64, t1362: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t53897, t53901, t53903, t53906, t53907, t53909, t53910) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2622(t12339, t5314, t1831, t40059, t16336, t3872, t16060, t3865, t1369, t16123, t68, t1362);
    (t53897, t53901, t53903, t53906, t53907, t53909, t53910)
}
