//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 955/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk955(t3868: f64, t5351: f64, t1658: f64, t449: f64, t863: f64, t864: f64, t1659: f64, t3896: f64, t4109: f64, t857: f64, t1265: f64, t4137: f64) -> (f64, f64, f64, f64, f64) {
    let t15210 = t3868 * t5351;
    let t15214 = t863 * t449 * t1658 * t864;
    let t15218 = t3896 * t1659;
    let t15221 = t857 * t4109;
    let t15223 = t4137 * t1265;
    (t15210, t15214, t15218, t15221, t15223)
}
