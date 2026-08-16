//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1760/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1760(t72: f64, t9940: f64, t245: f64, t543: f64, t9400: f64, t2713: f64, t3964: f64, t9714: f64, t3951: f64, t9732: f64, t1353: f64, t9994: f64) -> (f64, f64, f64, f64, f64) {
    let t47247 = t9940 * t72;
    let t47248 = t47247 * t245;
    let t47249 = t543 * t9400;
    let t47259 = t3964 * t2713 * t9714;
    let t47262 = t3964 * t9732 * t3951;
    let t47264 = t9994 * t1353;
    (t47248, t47249, t47259, t47262, t47264)
}
