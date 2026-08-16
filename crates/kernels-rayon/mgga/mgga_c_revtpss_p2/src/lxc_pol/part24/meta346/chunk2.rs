//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1205/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1205(t23740: f64, t23753: f64, t954: f64, t1621: f64, t19275: f64, t1634: f64, t6205: f64, t1633: f64, t19303: f64, t1610: f64, t6141: f64, t2874: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23754 = t23740 + t23753;
    let t23755 = t23754 * t954;
    let t23758 = t19275 * t1621;
    let t23761 = t1634 * t6205;
    let t23764 = t19303 * t1633;
    let t23767 = t1610 * t6141;
    let t23769 = 6.0_f64 * t2874 * t23767;
    (t23754, t23755, t23758, t23761, t23764, t23767, t23769)
}
