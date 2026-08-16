//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2255/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2255(t1089: f64, t23992: f64, t23997: f64, t24007: f64, t3304: f64, t3318: f64, t5004: f64, t6244: f64, t1082: f64, t24031: f64, t24111: f64, t23598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24132 = t23992 * t1089;
    let t24135 = t23997 * t1089;
    let t24138 = t24007 * t3304;
    let t24141 = t24007 * t3318;
    let t24144 = t5004 * t6244;
    let t24147 = t1082 * t24031;
    let t24152 = t24111 * t3318;
    let t24157 = t1082 * t23598;
    (t24132, t24135, t24138, t24141, t24144, t24147, t24152, t24157)
}
