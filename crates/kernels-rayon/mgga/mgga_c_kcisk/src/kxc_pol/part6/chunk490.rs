//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 490/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk490(t633: f64, t117: f64, t694: f64, t641: f64) -> (f64, f64, f64) {
    let t4956 = t633 * t633;
    let t4957 = 1.0_f64 / t4956;
    let t4969 = t117 * t694;
    let t4971 = 1.0_f64 / t641 / t4969;
    (t4956, t4957, t4971)
}
