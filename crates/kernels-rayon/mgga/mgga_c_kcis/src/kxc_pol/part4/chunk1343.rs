//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1343/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1343(t15849: f64, t15923: f64, t15985: f64, t16670: f64, t16734: f64, t16802: f64, t17004: f64, t17303: f64, t589: f64, t1505: f64, t5895: f64, t1555: f64) -> (f64, f64) {
    let t17306 = t15849 + t15923 + t15985 + t16670 + t16734 + t16802 + t17004 + t17303;
    let t17307 = t17306 * t589;
    let t17308 = t5895 * t1505;
    let t17310 = 2.0_f64 * t17308 * t1555;
    (t17307, t17310)
}
