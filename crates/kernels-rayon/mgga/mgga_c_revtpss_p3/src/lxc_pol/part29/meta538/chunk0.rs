//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1870/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1870(t2467: f64, t95743: f64, t93173: f64, t95725: f64, t93371: f64, t26488: f64, t686: f64, t72: f64, t93317: f64, t26492: f64, t25387: f64, t93281: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95744 = t95743 * t2467;
    let t95746 = t95725 * t93173;
    let t95747 = t93371 * t95746;
    let t95761 = t26488 * t72 * t686;
    let t95762 = t93317 * t95761;
    let t95765 = t26492 * t72 * t686;
    let t95766 = t25387 * t95765;
    let t95768 = t93281 * t95761;
    (t95744, t95746, t95747, t95762, t95765, t95766, t95768)
}
