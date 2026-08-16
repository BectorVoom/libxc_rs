//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1100/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1100(t10697: f64, t11669: f64, t11671: f64, t10698: f64, t11702: f64, t10885: f64, t11744: f64, t2834: f64, t3344: f64, t1615: f64, t3320: f64, t783: f64, t978: f64) -> (f64, f64, f64, f64, f64) {
    let t39502 = t10697 * t11669 * t11671;
    let t39511 = t10698 * t11702;
    let t39522 = t11744 * t10885;
    let t39548 = t2834 * t3344;
    let t39558 = t783 * t978 * t1615 * t3320;
    (t39502, t39511, t39522, t39548, t39558)
}
