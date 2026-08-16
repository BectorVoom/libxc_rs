//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1069/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1069(t2113: f64, t7334: f64, t1459: f64, t8731: f64, t1936: f64, t28974: f64, t572: f64, t26733: f64, t7002: f64, t7553: f64, t10301: f64, t8736: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32764 = 3.0_f64 * t2113 * t7334;
    let t32772 = 6.0_f64 * t1459 * t8731;
    let t32773 = t28974 * t1936;
    let t32775 = 6.0_f64 * t572 * t32773;
    let t32776 = t26733 * t1936;
    let t32778 = 6.0_f64 * t572 * t32776;
    let t32779 = t7553 * t7002;
    let t32781 = 6.0_f64 * t572 * t32779;
    let t32795 = t10301 * t8736;
    (t32764, t32772, t32773, t32775, t32776, t32778, t32779, t32781, t32795)
}
