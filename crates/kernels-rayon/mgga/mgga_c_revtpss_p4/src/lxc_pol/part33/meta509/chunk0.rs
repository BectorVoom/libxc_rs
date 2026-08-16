//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1829/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1829(t27212: f64, t786: f64, t7060: f64, t7063: f64, t14685: f64, t1941: f64, t14756: f64, t4435: f64, t7045: f64, t4426: f64, t7038: f64, t25245: f64, t4430: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27213 = t786 * t27212;
    let t27214 = t27213 * t7060;
    let t27216 = t7063 * t27212;
    let t27217 = t27216 * t7060;
    let t27221 = t1941 * t14685;
    let t27222 = t27221 * t14756;
    let t27224 = t7045 * t4435;
    let t27226 = t7038 * t4426;
    let t27228 = t25245 * t4430;
    (t27213, t27214, t27216, t27217, t27221, t27222, t27224, t27226, t27228)
}
