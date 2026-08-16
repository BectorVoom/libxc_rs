//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 551/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk551(t10615: f64, t9562: f64, t2365: f64, t7906: f64, t7025: f64, t1022: f64, t935: f64) -> (f64, f64, f64, f64, f64) {
    let t10616 = t10615 * t9562;
    let t10617 = 0.44688112439813033337e-1_f64 * t10616;
    let t10618 = t2365 * t7906;
    let t10619 = t7025 * t10618;
    let t10620 = 0.14896037479937677779e-1_f64 * t10619;
    let t10627 = t1022 * t935;
    (t10616, t10617, t10619, t10620, t10627)
}
