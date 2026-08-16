//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2547/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2547(t3298: f64, t4746: f64, t4891: f64, t12012: f64, t15822: f64, t1086: f64, t15654: f64, t3090: f64, t1025: f64, t371: f64, t4852: f64, t676: f64) -> (f64, f64, f64, f64) {
    let t53800 = t4746 * t3298 * t4891;
    let t53807 = t15822 * t12012;
    let t53855 = t15654 * t1086 * t3090;
    let t53875 = t1025 * t371 * t676 * t4852;
    (t53800, t53807, t53855, t53875)
}
