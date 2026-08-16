//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1060/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1060(t1936: f64, t26399: f64, t28658: f64, t7002: f64, t7359: f64, t2055: f64, t32392: f64, t93: f64, t7373: f64, t8692: f64, t25805: f64, t28025: f64, t32176: f64, t32178: f64, t32389: f64, t32609: f64, t670: f64, t6985: f64, t8564: f64) -> (f64, f64) {
    let t32642 = 2.0_f64 * t26399 * t1936;
    let t32644 = 2.0_f64 * t28658 * t1936;
    let t32646 = 2.0_f64 * t7359 * t7002;
    let t32654 = 2.0_f64 * t32392 * t2055;
    let t32655 = t93 * t7002;
    let t32657 = 2.0_f64 * t32655 * t2055;
    let t32659 = 2.0_f64 * t8692 * t7373;
    let t32660 = 2.0_f64 * t2055 * t25805 + 2.0_f64 * t2055 * t28025 + 2.0_f64 * t32389 * t670 + 2.0_f64 * t6985 * t7373 + t32176 + t32178 + t32609 + t32642 + t32644 + t32646 + t32654 + t32657 + t32659 + t8564;
    (t32655, t32660)
}
