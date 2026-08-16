//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1691;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta323(t12110: f64, t2375: f64, t3684: f64, t9882: f64, t9888: f64, t9885: f64, t3824: f64, t588: f64, t1287: f64, t2225: f64, t3681: f64, t750: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12111, t12114, t12116, t12118, t12120, t12123, t12126) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1691(t12110, t2375, t3684, t9882, t9888, t9885, t3824, t588, t1287, t2225, t3681, t750);
    (t12111, t12114, t12116, t12118, t12120, t12123, t12126)
}
