//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1024/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1024(t10785: f64, t2747: f64, t2749: f64, t125: f64, t2645: f64, t4364: f64, t4366: f64, t837: f64, t820: f64, t823: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10794 = t2747 * t10785 * t2749;
    let t10797 = t125 * t2645;
    let t10799 = t4364 * t10797 * t4366;
    let t10803 = t2747 * t10797 * t2749;
    let t10807 = t4364 * t10797 * t837;
    let t10811 = t820 * t823 * t844;
    (t10794, t10797, t10799, t10803, t10807, t10811)
}
