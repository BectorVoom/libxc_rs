//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2723/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2723(t2609: f64, t4395: f64, t14341: f64, t2398: f64, t40145: f64, t11084: f64, t15078: f64, t40141: f64, t4433: f64, t4541: f64, t50080: f64, t50085: f64, t50091: f64, t50093: f64, t50095: f64, t50096: f64) -> (f64, f64, f64, f64) {
    let t50097 = t4395 * t2609;
    let t50098 = 3.0_f64 * t50097;
    let t50099 = t2398 * t14341;
    let t50100 = 24.0_f64 * t50099;
    let t50101 = 12.0_f64 * t40145;
    let t50102 = -18.0_f64 * t11084 * t4433 * t4541 + 18.0_f64 * t15078 * t50080 + t40141 + t50085 + t50091 + t50093 + t50095 + t50096 + t50098 + t50100 + t50101;
    (t50098, t50100, t50101, t50102)
}
