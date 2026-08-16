//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1389/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1389(t3585: f64, t820: f64, t10401: f64, t3575: f64, t3610: f64, t3624: f64, t3521: f64, t3579: f64, t3577: f64, t248: f64, t3494: f64, t3570: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11668 = t820 * t3585;
    let t11677 = t3575 * t10401;
    let t11678 = t3610 * t11677;
    let t11692 = t3624 * t11677;
    let t11697 = t820 * t3521;
    let t11698 = t11697 * t3579;
    let t11699 = t3577 * t11698;
    let t11702 = t248 * t3570 * t3494;
    (t11668, t11678, t11692, t11697, t11698, t11699, t11702)
}
