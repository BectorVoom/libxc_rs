//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3615/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3615(t12254: f64, t141: f64, t68345: f64, t43764: f64, t68308: f64, t1145: f64, t68295: f64, t20349: f64, t698: f64, t20352: f64, t68299: f64, t68303: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t68529 = t141 * t12254 * t68345;
    let t68532 = t141 * t43764 * t68308;
    let t68535 = t141 * t1145 * t68295;
    let t68538 = t698 * t20349;
    let t68540 = t698 * t20352;
    let t68543 = t141 * t1145 * t68299;
    let t68546 = t141 * t1145 * t68303;
    (t68529, t68532, t68535, t68538, t68540, t68543, t68546)
}
