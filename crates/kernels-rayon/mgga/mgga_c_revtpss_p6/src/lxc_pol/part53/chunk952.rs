//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 952/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk952(t1936: f64, t27123: f64, t4292: f64, t93: f64, t7002: f64, t7889: f64, t2322: f64, t7741: f64, t5523: f64, t1312: f64, t28042: f64, t1518: f64, t25805: f64, t28025: f64, t28030: f64, t28160: f64, t28212: f64, t28214: f64, t28216: f64, t670: f64, t6985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28218 = 2.0_f64 * t27123 * t1936;
    let t28219 = t93 * t4292;
    let t28221 = 2.0_f64 * t28219 * t1936;
    let t28223 = 2.0_f64 * t7889 * t7002;
    let t28225 = 2.0_f64 * t2322 * t7741;
    let t28227 = 2.0_f64 * t5523 * t7741;
    let t28229 = 2.0_f64 * t1312 * t28042;
    let t28230 = 2.0_f64 * t1518 * t25805 + 2.0_f64 * t1518 * t28025 + 2.0_f64 * t28030 * t670 + 2.0_f64 * t4292 * t6985 + t28160 + t28212 + t28214 + t28216 + t28218 + t28221 + t28223 + t28225 + t28227 + t28229;
    (t28218, t28219, t28221, t28223, t28225, t28227, t28229, t28230)
}
