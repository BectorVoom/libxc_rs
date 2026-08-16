//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2346/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2346(t1263: f64, t5245: f64, t1122: f64, t1042: f64, t1234: f64, t5390: f64) -> (f64, f64, f64, f64) {
    let t17500 = t1263 * t5245;
    let t17501 = t17500 * t1122;
    let t17502 = t1042 * t17501;
    let t17505 = t1234 * t5390;
    (t17500, t17501, t17502, t17505)
}
