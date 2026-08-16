//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1962/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1962(t26756: f64, t98069: f64, t1877: f64, t2219: f64, t7845: f64, t2752: f64, t29105: f64, t24191: f64, t99053: f64, t1408: f64, t2057: f64, t24339: f64, t25028: f64, t2522: f64, t25381: f64, t26563: f64, t26740: f64, t26744: f64, t28456: f64, t28462: f64, t29106: f64, t6542: f64, t6671: f64, t7114: f64, t84800: f64, t98012: f64, t98020: f64, t98086: f64, t98112: f64, t99060: f64) -> (f64, f64, f64, f64, f64) {
    let t101211 = 2.0_f64 * t26756 * t98069;
    let t101220 = 2.0_f64 * t1877 * t7845 * t2219;
    let t101226 = t29105 * t2752;
    let t101241 = 6.0_f64 * t24191 * t99053;
    let t101248 = -t101211 + 3.0_f64 * t2522 * t2057 * t98020 - t1877 * t24339 * t28462 / 2.0_f64 + t101220 + t1877 * t84800 * t28456 + 3.0_f64 / 2.0_f64 * t2522 * t29106 * t6542 - t1877 * t101226 * t6671 / 2.0_f64 + 3.0_f64 * t2522 * t7845 * t25028 - t1877 * t7114 * t98086 / 2.0_f64 - t1877 * t26744 * t25381 - 3.0_f64 / 2.0_f64 * t24191 * t98012 + t101241 + t1877 * t26740 * t1408 + 6.0_f64 * t26563 * t99060 + 6.0_f64 * t24191 * t98112;
    (t101211, t101220, t101226, t101241, t101248)
}
