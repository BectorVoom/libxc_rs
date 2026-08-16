//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1351/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1351(t12524: f64, t33193: f64, t4072: f64, t576: f64, t1395: f64, t1458: f64, t26135: f64, t7230: f64, t7015: f64, t94170: f64, t24465: f64, t26550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120818 = 27.0_f64 * t12524 * t33193;
    let t120833 = t576 * t4072;
    let t120849 = t1395 * t1458;
    let t120865 = 0.135e2_f64 * t7230 * t26135;
    let t120867 = 27.0_f64 * t94170 * t7015;
    let t120869 = 27.0_f64 * t24465 * t26550;
    (t120818, t120833, t120849, t120865, t120867, t120869)
}
