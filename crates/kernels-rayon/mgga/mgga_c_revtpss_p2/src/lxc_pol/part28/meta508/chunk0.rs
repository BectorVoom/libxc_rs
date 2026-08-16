//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1900/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1900(t14756: f64, t27221: f64, t4435: f64, t7045: f64, t4426: f64, t7038: f64, t25245: f64, t4430: f64, t1561: f64, t25266: f64, t25270: f64, t4462: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27222 = t27221 * t14756;
    let t27224 = t7045 * t4435;
    let t27226 = t7038 * t4426;
    let t27228 = t25245 * t4430;
    let t27230 = t25266 * t1561;
    let t27232 = t25270 * t4462;
    (t27222, t27224, t27226, t27228, t27230, t27232)
}
