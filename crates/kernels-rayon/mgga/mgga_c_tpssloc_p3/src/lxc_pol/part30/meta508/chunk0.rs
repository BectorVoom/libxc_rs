//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1827/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1827(t360: f64, t4649: f64, t68: f64, t6744: f64, t344: f64, t7573: f64, t6740: f64, t1622: f64, t23489: f64, t23533: f64, t23537: f64, t23541: f64, t23544: f64, t23554: f64, t23560: f64, t4590: f64, t4596: f64, t4600: f64, t4636: f64, t4652: f64, t6723: f64, t6735: f64, t6742: f64, t6747: f64, t6755: f64, t6765: f64, t7574: f64, t7578: f64, t7583: f64) -> (f64, f64, f64, f64, f64) {
    let t25678 = t4649 * t68 * t360;
    let t25679 = t6744 * t25678;
    let t25682 = t7573 * t344;
    let t25683 = t6740 * t25682;
    let t25703 = 5.0_f64 / 6912.0_f64 * t6765 * t4590 + 0.10093189023535097714e-3_f64 * t23489 * t7583 + 0.10093189023535097714e-3_f64 * t6742 * t25679 + 0.10093189023535097714e-3_f64 * t25683 * t6747 + t23537 * t4596 / 768.0_f64 - t23541 * t4600 / 1536.0_f64 + t23533 / 3456.0_f64 + 0.80745512188280781712e-3_f64 * t6723 * t7578 - 0.10093189023535097714e-3_f64 * t7574 * t6735 + t23554 / 2304.0_f64 - t23560 / 432.0_f64 + t6755 * t4652 / 1536.0_f64 + t23544 * t1622 / 2304.0_f64 + t6765 * t4636 / 2304.0_f64;
    (t25678, t25679, t25682, t25683, t25703)
}
