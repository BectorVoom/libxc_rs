//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1362/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1362(t40406: f64, t685: f64, t827: f64, t837: f64, t10837: f64, t9775: f64, t10828: f64, t2741: f64, t10818: f64, t221: f64, t10703: f64, t2674: f64) -> (f64, f64, f64, f64) {
    let t40409 = t40406 * t827 * t685 * t837;
    let t40411 = t9775 * t10837;
    let t40413 = t2741 * t10828;
    let t40419 = t221 * t10818;
    let t40421 = t2674 * t10703 * t40419;
    (t40409, t40411, t40413, t40421)
}
