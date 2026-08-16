//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2485/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2485(t2659: f64, t4086: f64, t816: f64, t1412: f64, t808: f64, t1389: f64, t14224: f64, t46835: f64, t13769: f64, t2453: f64, t547: f64, t9794: f64) -> (f64, f64, f64, f64) {
    let t48862 = t816 * t2659 * t4086;
    let t48863 = t808 * t1412;
    let t48868 = t46835 * t1389 * t14224;
    let t48869 = 0.76230004213927992336e-5_f64 * t48868;
    let t48872 = t2453 * t547 * t9794 * t13769;
    (t48862, t48863, t48869, t48872)
}
