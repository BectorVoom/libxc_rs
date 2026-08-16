//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 956/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk956(t1936: f64, t25805: f64, t28025: f64, t6985: f64, t7002: f64, t648: f64, t8453: f64, t8692: f64, t2322: f64, t8460: f64, t5523: f64, t32161: f64, t32162: f64, t670: f64, t8564: f64) -> (f64, f64, f64, f64) {
    let t32165 = t25805 * t1936;
    let t32167 = t28025 * t1936;
    let t32169 = t6985 * t7002;
    let t32171 = t648 * t8453;
    let t32172 = 2.0_f64 * t32171;
    let t32174 = 4.0_f64 * t8692 * t7002;
    let t32175 = t2322 * t8460;
    let t32176 = 2.0_f64 * t32175;
    let t32177 = t5523 * t8460;
    let t32178 = 2.0_f64 * t32177;
    let t32179 = 2.0_f64 * t32162 * t670 + t32161 + 4.0_f64 * t32165 + 4.0_f64 * t32167 + 4.0_f64 * t32169 + t32172 + t32174 + t32176 + t32178 + t8564;
    (t32171, t32176, t32178, t32179)
}
