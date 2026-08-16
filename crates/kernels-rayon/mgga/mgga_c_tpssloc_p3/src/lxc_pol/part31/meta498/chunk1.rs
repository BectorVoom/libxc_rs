//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1694/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1694(t28088: f64, t559: f64, t6422: f64, t6945: f64, t6427: f64, t6952: f64, t6431: f64, t1831: f64, t26257: f64, t1799: f64, t1824: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28089 = t28088 * t559;
    let t28091 = t6945 * t6422;
    let t28093 = t6952 * t6427;
    let t28095 = t6952 * t6431;
    let t28097 = t26257 * t1831;
    let t28099 = t1799 * t1824;
    let t28100 = t28099 * t550;
    (t28089, t28091, t28093, t28095, t28097, t28100)
}
