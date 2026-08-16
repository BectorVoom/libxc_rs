//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3082/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3082(t12254: f64, t141: f64, t81160: f64, t43764: f64, t81212: f64, t3417: f64, t81182: f64, t1145: f64, t81198: f64, t81202: f64, t81190: f64, t81194: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81439 = t141 * t12254 * t81160;
    let t81442 = t141 * t43764 * t81212;
    let t81445 = t141 * t3417 * t81182;
    let t81448 = t141 * t1145 * t81198;
    let t81451 = t141 * t1145 * t81202;
    let t81454 = t141 * t1145 * t81190;
    let t81457 = t141 * t1145 * t81194;
    (t81439, t81442, t81445, t81448, t81451, t81454, t81457)
}
