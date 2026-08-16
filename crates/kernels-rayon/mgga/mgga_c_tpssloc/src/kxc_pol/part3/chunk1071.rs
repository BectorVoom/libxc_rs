//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1071/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1071(t13861: f64, t2988: f64, t13542: f64, t4518: f64, t13546: f64, t10259: f64, t4514: f64, t13559: f64, t13555: f64, t4510: f64, t1597: f64, t3014: f64, t343: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13862 = t2988 * t13861;
    let t13865 = t4518 * t13542;
    let t13868 = t4518 * t13546;
    let t13871 = t10259 * t4514;
    let t13874 = t4518 * t13559;
    let t13877 = t4510 * t13555;
    let t13881 = t1597 * t3014 * t343;
    (t13862, t13865, t13868, t13871, t13874, t13877, t13881)
}
