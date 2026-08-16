//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 759/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk759(t5: f64, t2035: f64, t8764: f64, t2042: f64, t2170: f64, t2121: f64, t136: f64, t8442: f64, t8436: f64, t117: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t8765 = t8764 * t2035;
    let t8773 = t2170 * t2042;
    let t8911 = t2121 * t2121;
    let t8912 = t8911 * t136;
    let t8913 = t8912 * t8442;
    let t8916 = piecewise3(t8, 0.0_f64, 5.0_f64 / 144.0_f64 * t8436 * t8913);
    let t8917 = t8916 * t117;
    (t8765, t8773, t8911, t8912, t8913, t8916, t8917)
}
