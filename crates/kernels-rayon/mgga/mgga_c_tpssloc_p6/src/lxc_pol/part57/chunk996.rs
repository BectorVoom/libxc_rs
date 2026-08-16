//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 996/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk996(t1880: f64, t1894: f64, t214: f64, t29040: f64, t113005: f64, t114673: f64, t114689: f64, t114694: f64, t121536: f64, t126456: f64, t126472: f64, t126476: f64, t126477: f64, t5575: f64, t8560: f64) -> f64 {
    let t127995 = t1880 * t214 * t1894 * t29040;
    let t127998 = -t126456 + t114673 - t126472 - t126476 + 0.38381794893125283518e-1_f64 * t121536 + t126477 - t113005 + 0.82246703342411321825e-2_f64 * t127995 - t114689 + t114694 + t5575 * t8560;
    t127998
}
