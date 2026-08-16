//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2975/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2975(t14077: f64, t4630: f64, t10401: f64, t246: f64, t3067: f64, t3186: f64, t1615: f64, t3061: f64, t375: f64, t1022: f64, t3961: f64, t3200: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t62049 = t14077 * t4630;
    let t62053 = t10401 * t246;
    let t62054 = t3067 * t62053;
    let t62055 = t3186 * t62054;
    let t62057 = t375 * t3061 * t1615;
    let t62059 = t3961 * t1022;
    let t62064 = t3200 * t62054;
    (t62049, t62053, t62055, t62057, t62059, t62064)
}
