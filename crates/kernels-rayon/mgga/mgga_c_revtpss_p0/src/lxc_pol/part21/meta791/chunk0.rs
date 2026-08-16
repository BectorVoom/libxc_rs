//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2850/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2850(t42518: f64, t51959: f64, t52011: f64, t42731: f64, t2852: f64, t346: f64, t2889: f64, t918: f64, t15107: f64, t15110: f64, t128: f64, t2850: f64, t51993: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52013 = t52011 * t42518 * t51959;
    let t52016 = t52011 * t42731 * t51959;
    let t52018 = t346 * t2852;
    let t52020 = t52011 * t52018 * t51959;
    let t52022 = t918 * t2889;
    let t52023 = t15107 * t52022;
    let t52025 = t15110 * t52022;
    let t52028 = t128 * t2850 * t51993;
    (t52013, t52016, t52020, t52023, t52025, t52028)
}
