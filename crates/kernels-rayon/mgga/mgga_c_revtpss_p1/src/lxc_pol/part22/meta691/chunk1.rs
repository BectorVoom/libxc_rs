//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2693/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2693(t22079: f64, t3936: f64, t3938: f64, t1399: f64, t5673: f64, t21990: f64, t5674: f64, t13944: f64, t6869: f64, t543: f64, t5591: f64) -> (f64, f64, f64, f64, f64) {
    let t22081 = t3936 * t22079 * t3938;
    let t22085 = t5673 * t22079 * t1399;
    let t22089 = t5673 * t5674 * t21990;
    let t22093 = t3936 * t13944 * t6869;
    let t22096 = t543 * t5591;
    (t22081, t22085, t22089, t22093, t22096)
}
