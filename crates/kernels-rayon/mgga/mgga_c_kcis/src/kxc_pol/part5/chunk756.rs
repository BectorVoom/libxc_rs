//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 756/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk756(t1307: f64, t5632: f64, t1395: f64, t1394: f64, t1397: f64, t5752: f64, t1947: f64, t3738: f64, t1392: f64, t4992: f64, t86: f64, t1396: f64, t5477: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5769 = t5632 * t1307;
    let t5770 = t1395 * t5769;
    let t5771 = t1394 * t5770;
    let t5773 = t5752 * t1397;
    let t5774 = t1394 * t5773;
    let t5776 = t3738 * t1947;
    let t5777 = t1394 * t5776;
    let t5780 = t86 * t4992 * t1392;
    let t5781 = t1396 * t5477;
    (t5769, t5770, t5771, t5773, t5774, t5776, t5777, t5780, t5781)
}
