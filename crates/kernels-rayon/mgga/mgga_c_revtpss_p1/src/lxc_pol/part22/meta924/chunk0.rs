//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3146/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3146(t1204: f64, t5412: f64, t1811: f64, t3552: f64, t1269: f64, t17288: f64, t3555: f64, t5216: f64, t3565: f64, t5215: f64, t487: f64, t3566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t56503 = t1204 * t5412;
    let t56508 = t3552 * t1811;
    let t56519 = t17288 * t1269;
    let t56570 = t3555 * t5412;
    let t56575 = t5216 * t1269;
    let t56587 = t5215 * t3565;
    let t56588 = t56587 * t487;
    let t56607 = t3566 * t5412;
    (t56503, t56508, t56519, t56570, t56575, t56587, t56588, t56607)
}
