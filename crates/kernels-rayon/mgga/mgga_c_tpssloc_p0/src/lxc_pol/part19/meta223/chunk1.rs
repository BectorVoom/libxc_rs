//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 927/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk927(t3053: f64, t3117: f64, t1043: f64, t676: f64, t248: f64, t884: f64, t1041: f64, t3048: f64, t10478: f64, t3128: f64, t10472: f64, t10481: f64, t3131: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10866 = t3117 * t3053;
    let t10868 = t676 * t1043;
    let t10870 = t248 * t10868 * t884;
    let t10871 = t1041 * t10870;
    let t10873 = t3048 * t3053;
    let t10875 = t3128 * t10478;
    let t10876 = t10472 * t10875;
    let t10877 = t10481 * t3131;
    (t10866, t10868, t10870, t10871, t10873, t10875, t10876, t10877)
}
