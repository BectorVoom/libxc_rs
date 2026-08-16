//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1532/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1532(t11858: f64, t15688: f64, t16102: f64, t3155: f64, t1020: f64, t12003: f64, t12077: f64, t15905: f64, t994: f64, t3075: f64, t3154: f64, t11671: f64, t11865: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43082 = t11858 * t15688;
    let t43085 = t3155 * t16102;
    let t43091 = t1020 * t12003;
    let t43105 = t994 * t12077 * t15905;
    let t43116 = t3154 * t3075;
    let t43121 = t11865 * t11671;
    (t43082, t43085, t43091, t43105, t43116, t43121)
}
