//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1000/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1000(t115545: f64, t22633: f64, t26338: f64, t120240: f64, t22635: f64, t31558: f64, t26331: f64, t31549: f64, t5308: f64, t1985: f64, t26193: f64, t31607: f64) -> (f64, f64, f64, f64) {
    let t122213 = t22633 * t115545 * t26338;
    let t122218 = t22633 * t22635 * t31558 * t120240;
    let t122227 = t26331 * t22635 * t31549 * t5308;
    let t122235 = t1985 * t26193 * t31607;
    (t122213, t122218, t122227, t122235)
}
