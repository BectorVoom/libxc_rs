//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1394/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1394(t10165: f64, t18070: f64, t225: f64, t5915: f64, t1049: f64, t5872: f64, t3201: f64, t3188: f64, t1057: f64, t18028: f64, t1615: f64, t4657: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18071 = t10165 * t18070;
    let t18074 = t5915 * t225;
    let t18080 = t1049 * t5872;
    let t18081 = t18080 * t3201;
    let t18083 = t18080 * t3188;
    let t18086 = t18028 * t1057;
    let t18088 = t4657 * t1615;
    (t18071, t18074, t18081, t18083, t18086, t18088)
}
