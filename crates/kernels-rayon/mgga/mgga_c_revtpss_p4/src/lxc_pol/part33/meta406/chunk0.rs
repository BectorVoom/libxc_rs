//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1457/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1457(t473: f64, t5412: f64, t13147: f64, t487: f64, t460: f64, t12050: f64, t13045: f64, t13141: f64, t3603: f64, t1284: f64, t5216: f64, t1770: f64, t3766: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17821 = t473 * t5412;
    let t17845 = t13147 * t487;
    let t17846 = t460 * t17845;
    let t17847 = t12050 * t13045;
    let t17852 = t13141 * t487;
    let t17853 = t460 * t17852;
    let t17854 = t12050 * t3603;
    let t17861 = t5216 * t1284;
    let t17934 = t1770 * t3766;
    (t17821, t17846, t17847, t17853, t17854, t17861, t17934)
}
