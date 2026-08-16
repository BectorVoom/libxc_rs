//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1526/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1526(t2940: f64, t4498: f64, t2925: f64, t4488: f64, t959: f64, t1634: f64, t3175: f64, t10165: f64, t1065: f64, t4693: f64, t3174: f64, t2970: f64, t4343: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13731 = 0.34631718211362927518e2_f64 * t2940 * t4498;
    let t13732 = t4488 * t2925;
    let t13734 = 0.11696447245269292414e1_f64 * t959 * t13732;
    let t13735 = t1634 * t3175;
    let t13736 = t10165 * t13735;
    let t13742 = t4693 * t1065;
    let t13743 = t3174 * t13742;
    let t13748 = t2970 * t4343;
    (t13731, t13734, t13735, t13736, t13742, t13743, t13748)
}
