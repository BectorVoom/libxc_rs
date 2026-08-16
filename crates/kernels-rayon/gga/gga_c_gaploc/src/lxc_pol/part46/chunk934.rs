//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 934/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk934(t7064: f64, t7069: f64, t8878: f64, t161: f64, t1841: f64, t2576: f64, t33137: f64, t13212: f64, t7129: f64, t10789: f64, t2508: f64, t2586: f64) -> (f64, f64, f64, f64) {
    let t43042 = t7064 * t8878 * t7069;
    let t43043 = 0.1922631557535556071e-2_f64 * t43042;
    let t43046 = t1841 * t33137 * t161 * t2576;
    let t43049 = 0.23071578690426672851e-1_f64 * t7129 * t13212;
    let t43051 = t2508 * t10789 * t2586;
    (t43043, t43046, t43049, t43051)
}
