//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 958/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk958(t1041: f64, t10870: f64, t3048: f64, t3053: f64, t10478: f64, t3128: f64, t10472: f64, t10481: f64, t3131: f64, t1021: f64, t248: f64, t1015: f64) -> (f64, f64, f64, f64, f64) {
    let t10871 = t1041 * t10870;
    let t10873 = t3048 * t3053;
    let t10875 = t3128 * t10478;
    let t10876 = t10472 * t10875;
    let t10877 = t10481 * t3131;
    let t10879 = t248 * t1021 * t10877;
    let t10882 = t1015 * t10478;
    (t10871, t10873, t10876, t10879, t10882)
}
