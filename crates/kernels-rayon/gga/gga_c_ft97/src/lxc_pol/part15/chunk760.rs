//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 760/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk760(t21181: f64, t9652: f64, t420: f64, t701: f64, t20489: f64, t704: f64, t3799: f64, t5042: f64, t9657: f64, t3690: f64, t4635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21182 = t9652 * t21181;
    let t21183 = t420 * t21182;
    let t21184 = t701 * t21183;
    let t21186 = t704 * t20489;
    let t21187 = t420 * t21186;
    let t21188 = t701 * t21187;
    let t21190 = t3799 * t5042;
    let t21192 = t9657 * t21181;
    let t21193 = t420 * t21192;
    let t21194 = t701 * t21193;
    let t21196 = t3690 * t4635;
    (t21182, t21183, t21184, t21186, t21187, t21188, t21190, t21192, t21193, t21194, t21196)
}
