//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1072/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1072(t1936: f64, t28974: f64, t572: f64, t26733: f64, t7002: f64, t7553: f64, t1461: f64, t2040: f64, t2115: f64, t32373: f64, t32377: f64, t32755: f64, t32760: f64, t32762: f64, t32764: f64, t32772: f64, t573: f64, t7324: f64, t7554: f64, t7557: f64, t8616: f64, t8725: f64) -> (f64, f64, f64, f64) {
    let t32773 = t28974 * t1936;
    let t32775 = 6.0_f64 * t572 * t32773;
    let t32776 = t26733 * t1936;
    let t32778 = 6.0_f64 * t572 * t32776;
    let t32779 = t7553 * t7002;
    let t32781 = 6.0_f64 * t572 * t32779;
    let t32782 = 3.0_f64 * t1461 * t8725 + 6.0_f64 * t2040 * t7554 + 3.0_f64 * t2040 * t7557 + 3.0_f64 * t2115 * t7324 + t32755 * t573 + t32373 + t32377 + t32760 + t32762 + t32764 + t32772 + t32775 + t32778 + t32781 + t8616;
    (t32773, t32776, t32779, t32782)
}
