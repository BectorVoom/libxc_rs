//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 541/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk541(t22632: f64, t5598: f64, t5599: f64, t172: f64, t5590: f64, t5592: f64, t5587: f64, t397: f64, t39: f64, t78: f64, t388: f64, t5517: f64, t66: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22634 = t5598 * t22632 * t5599;
    let t22642 = t5590 * t172;
    let t22643 = t22642 * t5592;
    let t22644 = t5587 * t22643;
    let t22652 = sigma0 * t397;
    let t22686 = t78 * t39;
    let t22687 = t388 * t22686;
    let t22696 = t5517 * t66;
    (t22634, t22643, t22644, t22652, t22687, t22696)
}
