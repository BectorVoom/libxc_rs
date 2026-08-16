//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 905/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk905(t2430: f64, t8746: f64, t1746: f64, t4954: f64, t8763: f64, t7156: f64, t10913: f64, t4957: f64, t1248: f64, t28377: f64, t4893: f64, t1720: f64, t28385: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29102 = t8746 * t2430;
    let t29104 = t4954 * t29102 * t1746;
    let t29107 = t1746 * t8763;
    let t29108 = t7156 * t29107;
    let t29111 = t10913 * t29102;
    let t29112 = t29111 * t4957;
    let t29116 = t1248 * t4893 * t28377;
    let t29121 = t1248 * t1720 * t28385;
    (t29102, t29104, t29108, t29112, t29116, t29121)
}
