//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1811/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1811(t25040: f64, t82074: f64, t87712: f64, t82294: f64, t25193: f64, t81591: f64, t28: f64, t40772: f64, t1649: f64, t2752: f64, t1437: f64, t6509: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87927 = t87712 * t82074 * t25040;
    let t87929 = 0.10417915756705434098e0_f64 * t82294;
    let t87931 = t81591 * t25193;
    let t89953 = t40772 * t28;
    let t89992 = t2752 * t1649;
    let t90090 = t6509 * t1437;
    (t87927, t87929, t87931, t89953, t89992, t90090)
}
