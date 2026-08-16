//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1621/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1621(t3566: f64, t3766: f64, t5330: f64, t3568: f64, t3601: f64, t12646: f64, t12915: f64, t247: f64, t5384: f64, t12831: f64, t12865: f64, t1260: f64, t12889: f64) -> (f64, f64, f64, f64, f64) {
    let t44550 = t3566 * t3766;
    let t44551 = t44550 * t5330;
    let t44552 = t3568 * t3601;
    let t44559 = t5384 * t247 * t12915 * t12646;
    let t44561 = t12831 * t12865;
    let t44568 = t12889 * t1260;
    (t44551, t44552, t44559, t44561, t44568)
}
