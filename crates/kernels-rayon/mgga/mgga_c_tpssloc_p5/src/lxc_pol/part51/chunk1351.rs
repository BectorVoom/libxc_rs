//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1351/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1351(t16521: f64, t8326: f64, t12524: f64, t33193: f64, t4072: f64, t576: f64, t1395: f64, t1458: f64, t33662: f64, t26135: f64, t7230: f64, t7015: f64, t94170: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t120809 = 0.135e2_f64 * t16521 * t8326;
    let t120818 = 27.0_f64 * t12524 * t33193;
    let t120833 = t576 * t4072;
    let t120849 = t1395 * t1458;
    let t120857 = t576 * t33662;
    let t120865 = 0.135e2_f64 * t7230 * t26135;
    let t120867 = 27.0_f64 * t94170 * t7015;
    (t120809, t120818, t120833, t120849, t120857, t120865, t120867)
}
