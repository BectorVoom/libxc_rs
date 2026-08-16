//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 997/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk997(t524: f64, t943: f64, t944: f64, t1219: f64, t615: f64, t8396: f64, t525: f64, t847: f64, t448: f64, t315: f64, t2137: f64, t33428: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33750 = t524 * t943;
    let t33751 = t33750 * t944;
    let t33778 = t615 * t8396 * t1219;
    let t33787 = t525 * t847;
    let t33795 = t8396 * t448;
    let t33796 = t315 * t33795;
    let t33799 = t2137 * t33795;
    let t33802 = t315 * t33428;
    (t33750, t33751, t33778, t33787, t33796, t33799, t33802)
}
