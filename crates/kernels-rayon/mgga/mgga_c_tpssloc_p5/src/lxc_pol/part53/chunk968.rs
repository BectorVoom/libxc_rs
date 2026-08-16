//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 968/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk968(t30714: f64, t4240: f64, t4250: f64, t4191: f64, t1484: f64, t865: f64, t23788: f64, t4255: f64, t25365: f64, t25927: f64, t25374: f64, t89953: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t118608 = t30714 * t4240;
    let t118610 = t30714 * t4250;
    let t118612 = t30714 * t4191;
    let t118833 = t1484 * t865;
    let t119691 = t23788 * t4255;
    let t119713 = t25927 * t25365;
    let t119755 = t89953 * t25374;
    (t118608, t118610, t118612, t118833, t119691, t119713, t119755)
}
