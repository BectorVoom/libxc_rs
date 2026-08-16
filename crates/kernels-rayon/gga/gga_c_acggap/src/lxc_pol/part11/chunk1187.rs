//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1187/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1187(t17912: f64, t31443: f64, t33953: f64, t5207: f64, t142: f64, t5160: f64, t7436: f64, t2030: f64, t4495: f64, t7815: f64, t2060: f64, t5187: f64) -> (f64, f64, f64, f64) {
    let t36250 = t31443 * t17912 * t33953 * t5207;
    let t36253 = t7436 * t142 * t5160;
    let t36256 = t2030 * t7815 * t4495;
    let t36259 = t2060 * t7815 * t5187;
    (t36250, t36253, t36256, t36259)
}
