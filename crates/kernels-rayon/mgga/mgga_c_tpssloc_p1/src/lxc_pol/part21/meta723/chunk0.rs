//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2578/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2578(t14783: f64, t699: f64, t14786: f64, t14789: f64, t14778: f64, t11153: f64, t1229: f64, t3242: f64, t486: f64, t11147: f64, t3584: f64, t2403: f64, t4775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t50968 = t699 * t14783;
    let t50970 = t699 * t14786;
    let t50972 = t699 * t14789;
    let t50978 = t699 * t14778;
    let t50992 = t1229 * t11153;
    let t50998 = t486 * t3242;
    let t51002 = t3584 * t11147;
    let t51039 = t2403 * t4775;
    (t50968, t50970, t50972, t50978, t50992, t50998, t51002, t51039)
}
