//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 709/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk709(t1891: f64, t5448: f64, t653: f64, t219: f64, t518: f64, t201: f64, t673: f64, t681: f64, t1932: f64, t1966: f64, t207: f64, t1931: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5479 = 0.57895126195293126241e3_f64 * t1891 * t653 * t5448;
    let t5486 = t518 * t219;
    let t5490 = t518 * t201;
    let t5503 = t673 * t681;
    let t5504 = t5503 * t1932;
    let t5507 = t207 * t1966;
    let t5508 = t1931 * t5507;
    (t5479, t5486, t5490, t5504, t5507, t5508)
}
