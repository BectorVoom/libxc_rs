//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2798/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2798(t22352: f64, t2435: f64, t2782: f64, t4086: f64, t543: f64, t74965: f64, t4003: f64, t5744: f64, t74982: f64, t74700: f64, t4100: f64, t22394: f64, t686: f64, t72: f64, t9680: f64) -> (f64, f64, f64, f64, f64) {
    let t75274 = t2435 * t22352;
    let t75298 = t2782 * t4086 * t74965 * t543;
    let t75302 = t2782 * t5744 * t74982 * t4003;
    let t75305 = t74700 * t543;
    let t75307 = t2782 * t4100 * t75305;
    let t75336 = t9680 * t22394 * t72 * t686;
    (t75274, t75298, t75302, t75307, t75336)
}
