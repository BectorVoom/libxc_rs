//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1058/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1058(t31056: f64, t1266: f64, t8326: f64, t652: f64, t113: f64, t1869: f64, t1976: f64, t30989: f64, t30993: f64, t30995: f64, t31029: f64, t31034: f64, t31038: f64, t31039: f64, t31041: f64, t31046: f64, t31050: f64, t31052: f64, t31055: f64, t510: f64, t650: f64, t6515: f64, t6862: f64, t8313: f64, t8329: f64, t8439: f64) -> (f64, f64, f64, f64) {
    let t31057 = 2.0_f64 * t31056;
    let t31058 = t1266 * t8326;
    let t31059 = t652 * t31058;
    let t31060 = 2.0_f64 * t31059;
    let t31061 = -t113 * t30989 - t1266 * t8313 - 2.0_f64 * t1869 * t6862 - 2.0_f64 * t1976 * t6515 - t31029 * t510 - t650 * t8439 - t30993 - t30995 - t31034 - t31038 + 6.0_f64 * t31039 - 2.0_f64 * t31041 + t31046 + t31050 - 4.0_f64 * t31052 - t31055 - t31057 - t31060 - t8329;
    (t31057, t31058, t31060, t31061)
}
