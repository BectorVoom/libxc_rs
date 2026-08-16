//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2552/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2552(t43347: f64, t53668: f64, t11852: f64, t41270: f64, t3316: f64, t4746: f64, t4891: f64, t16381: f64, t3090: f64, t11262: f64, t3127: f64, t4874: f64) -> (f64, f64, f64, f64, f64) {
    let t54509 = t43347 * t53668;
    let t54537 = t11852 * t41270;
    let t54570 = t4746 * t3316 * t4891;
    let t54578 = t16381 * t3090;
    let t54599 = t3127 * t11262 * t4874;
    (t54509, t54537, t54570, t54578, t54599)
}
