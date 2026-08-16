//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 669/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk669(t14083: f64, t4765: f64, t49: f64, t68407: f64, t14030: f64, t14121: f64, t14123: f64) -> (f64, f64, f64, f64) {
    let t68417 = t4765 * t14083 * t49;
    let t68418 = t68417 * t68407;
    let t68420 = t14030 * t14121;
    let t68421 = t68420 * t14123;
    (t68417, t68418, t68420, t68421)
}
