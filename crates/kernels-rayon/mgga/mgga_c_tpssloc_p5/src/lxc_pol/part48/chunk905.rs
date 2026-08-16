//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 905/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk905(t1894: f64, t2553: f64, t59: f64, t6591: f64, t240: f64, t241: f64, t2627: f64, t812: f64, t2632: f64, t4180: f64, t9626: f64, t2617: f64, t30713: f64) -> (f64, f64, f64) {
    let t112788 = t6591 * t1894 * t59 * t2553;
    let t112792 = t812 * t2627 * t240 * t241;
    let t112795 = t112792 * t4180 * t9626 * t2632;
    let t112797 = t2617 * t30713;
    (t112788, t112795, t112797)
}
