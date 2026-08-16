//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1265/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1265(t6662: f64, t857: f64, t30667: f64, t6547: f64, t6562: f64, t82133: f64, t8335: f64, t23168: f64, t30664: f64, t30643: f64, t23109: f64, t23110: f64, t232: f64, t59: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t112719 = t857 * t6662;
    let t112726 = t6547 * t30667;
    let t112741 = t6562 * t82133 * t8335;
    let t112743 = t23168 * t30664;
    let t112760 = t6547 * t30643;
    let t112778 = t23109 * t23110 * t59 * t828 * t232;
    (t112719, t112726, t112741, t112743, t112760, t112778)
}
