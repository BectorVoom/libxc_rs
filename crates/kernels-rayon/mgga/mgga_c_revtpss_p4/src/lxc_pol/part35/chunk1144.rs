//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1144/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1144(t102385: f64, t94391: f64, t26292: f64, t27899: f64, t102295: f64, t7289: f64, t1426: f64, t786: f64, t8086: f64, t14090: f64, t26265: f64, t14104: f64, t96515: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102386 = t94391 * t102385;
    let t102409 = t27899 * t26292;
    let t102411 = t7289 * t102295;
    let t102420 = t786 * t8086 * t1426;
    let t102434 = t26265 * t14090;
    let t102439 = t96515 * t14104;
    (t102386, t102409, t102411, t102420, t102434, t102439)
}
