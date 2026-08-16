//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2466/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2466(t5603: f64, t9692: f64, t136: f64, t2457: f64, t5774: f64, t9674: f64, t10073: f64, t13731: f64, t3915: f64, t5721: f64, t9288: f64, t2439: f64, t3895: f64, t5775: f64) -> (f64, f64, f64, f64, f64) {
    let t47863 = t5603 * t9692;
    let t47885 = t9674 * t5774 * t136 * t2457;
    let t47886 = 0.34697458558045176417e-2_f64 * t47885;
    let t47899 = t10073 * t13731;
    let t47904 = t3915 * t5721 * t9288;
    let t47907 = t2439 * t3895 * t5775;
    (t47863, t47886, t47899, t47904, t47907)
}
