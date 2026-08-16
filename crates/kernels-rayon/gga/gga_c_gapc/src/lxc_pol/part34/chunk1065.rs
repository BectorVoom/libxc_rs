//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1065/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1065(t10099: f64, t3568: f64, t3846: f64, t972: f64, t1096: f64, t3622: f64, t2469: f64, t3832: f64, t7063: f64, t1125: f64, t3449: f64, t12039: f64, t12041: f64, t12042: f64, t12048: f64, t12049: f64, t12051: f64, t12057: f64, t12060: f64, t12064: f64, t12150: f64) -> (f64, f64, f64, f64, f64) {
    let t12281 = 2.0_f64 * t10099 * t3568;
    let t12282 = t3846 * t972;
    let t12285 = t3622 * t1096;
    let t12287 = 2.0_f64 * t2469 * t12285;
    let t12288 = t3832 * t972;
    let t12290 = 6.0_f64 * t7063 * t12288;
    let t12291 = t1125 * t3449;
    let t12293 = 2.0_f64 * t2469 * t12291;
    let t12294 = 2.0_f64 * t12282 * t2469 - t12039 + t12041 + t12042 - t12048 + t12049 - t12051 - t12057 - t12060 - t12064 + t12150 + t12281 + t12287 - t12290 + t12293;
    (t12282, t12285, t12288, t12291, t12294)
}
