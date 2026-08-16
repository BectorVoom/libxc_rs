//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1369/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1369(t17670: f64, t17671: f64, t4582: f64, t1539: f64, t4650: f64, t3071: f64, t5867: f64, t884: f64, t10390: f64, t1041: f64, t10480: f64, t10904: f64, t13995: f64, t14000: f64, t14027: f64, t17643: f64, t17649: f64, t17656: f64, t17660: f64, t17662: f64, t17668: f64, t3070: f64, t4575: f64, t5875: f64, t5909: f64) -> f64 {
    let t17672 = t17670 * t17671;
    let t17673 = t4582 * t17672;
    let t17676 = t4650 * t1539;
    let t17677 = t3071 * t17676;
    let t17680 = t5867 * t884;
    let t17681 = t3071 * t17680;
    let t17684 = 5.0_f64 / 13824.0_f64 * t1041 * t17643 + t13995 * t4575 / 2304.0_f64 - t3070 * t17649 / 2304.0_f64 + t10390 * t5909 / 2304.0_f64 - t17656 / 4608.0_f64 + t17660 / 6912.0_f64 + t17662 / 2304.0_f64 - t10904 * t5875 / 288.0_f64 + t17668 / 2304.0_f64 + t14000 + t10480 * t17673 / 512.0_f64 + t14027 + t3070 * t17677 / 2304.0_f64 + t3070 * t17681 / 4608.0_f64;
    t17684
}
