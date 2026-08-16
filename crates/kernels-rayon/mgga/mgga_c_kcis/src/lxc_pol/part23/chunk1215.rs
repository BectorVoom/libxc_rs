//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1215/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1215(t15956: f64, t97793: f64, t17383: f64, t7952: f64, t27517: f64, t5910: f64, t2060: f64, t1467: f64, t4294: f64, t1928: f64, t4254: f64, t27521: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t97794 = t97793 * t15956;
    let t97796 = t7952 * t17383;
    let t97798 = t27517 * t5910;
    let t97800 = sigma2 * t2060;
    let t97801 = t1467 * t97800;
    let t97802 = t97801 * t4294;
    let t97804 = t4254 * t1928;
    let t97805 = t97804 * t27521;
    (t97794, t97796, t97798, t97802, t97805)
}
