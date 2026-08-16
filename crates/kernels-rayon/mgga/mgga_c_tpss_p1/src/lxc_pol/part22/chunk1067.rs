//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1067/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1067(t11678: f64, t2741: f64, t1465: f64, t2460: f64, t8523: f64, t242: f64, t8469: f64, t946: f64, t1407: f64, t8951: f64, t967: f64, t2748: f64, t3969: f64) -> (f64, f64, f64, f64, f64) {
    let t11679 = t2741 * t11678;
    let t11682 = t1465 * t2460;
    let t11683 = t8523 * t11682;
    let t11687 = t242 * t8469 * t1465;
    let t11688 = t946 * t11687;
    let t11691 = t242 * t8951 * t1407;
    let t11692 = t967 * t11691;
    let t11697 = t2748 * t3969 / 648.0_f64;
    (t11679, t11683, t11688, t11692, t11697)
}
