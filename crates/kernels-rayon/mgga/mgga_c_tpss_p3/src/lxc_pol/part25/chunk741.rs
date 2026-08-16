//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 741/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk741(t4875: f64, t866: f64, t846: f64, t2533: f64, t4843: f64, t2531: f64, t2537: f64, t3746: f64, t4828: f64, t4832: f64, t4836: f64, t1436: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4876 = t4875 * t866;
    let t4878 = 1.0_f64 * t846 * t4876;
    let t4879 = t4843 * t2533;
    let t4881 = 0.16081979498692535067e2_f64 * t2531 * t4879;
    let t4886 = t2537 + 0.11415555555555555555e-1_f64 * t3746 - 0.11415555555555555555e-1_f64 * t4828 + 0.34246666666666666666e-1_f64 * t4832 - 0.17123333333333333333e-1_f64 * t4836;
    let t4891 = t1436 * t1436;
    (t4876, t4878, t4879, t4881, t4886, t4891)
}
