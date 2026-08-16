//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1265/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1265(t23185: f64, t30634: f64, t82074: f64, t6662: f64, t857: f64, t30667: f64, t6547: f64, t6562: f64, t82133: f64, t8335: f64, t23168: f64, t30664: f64) -> (f64, f64, f64, f64, f64) {
    let t112702 = t23185 * t82074 * t30634;
    let t112719 = t857 * t6662;
    let t112726 = t6547 * t30667;
    let t112741 = t6562 * t82133 * t8335;
    let t112743 = t23168 * t30664;
    (t112702, t112719, t112726, t112741, t112743)
}
