//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1417/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1417(t78064: f64, t78076: f64, t1107: f64, t43880: f64, t78028: f64, t43777: f64, t50846: f64, t71470: f64, t71472: f64, t71474: f64, t78026: f64, t78029: f64, t78033: f64, t78037: f64, t78041: f64, t78045: f64, t78049: f64) -> (f64, f64, f64, f64) {
    let t78077 = t78064 + t78076;
    let t78078 = t1107 * t78077;
    let t78080 = t43880 * t78028;
    let t78082 = -0.98115555555555555556e0_f64 * t50846 - 0.98115555555555555555e-1_f64 * t71470 + 0.44152e0_f64 * t71472 - 0.132456e1_f64 * t71474 + t43777 - 0.3883875e1_f64 * t78026 + 0.6189328125e-1_f64 * t78029 - 0.80513333333333333332e0_f64 * t78033 + 0.20128333333333333334e1_f64 * t78037 - 0.72462e1_f64 * t78041 + 0.108693e2_f64 * t78045 + 0.24154e1_f64 * t78049 + 0.16504875e0_f64 * t78078 - 0.485484375e1_f64 * t78080;
    (t78077, t78078, t78080, t78082)
}
