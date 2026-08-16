//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2494/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2494(t12800: f64, t3636: f64, t3551: f64, t3565: f64, t225: f64, t12884: f64, t828: f64, t12788: f64, t3625: f64, t12732: f64, t73: f64, t3555: f64, t3766: f64, t5330: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44418 = t12800 * t3636;
    let t44420 = t3551 * t3565;
    let t44421 = t44420 * t225;
    let t44425 = t828 * t12884;
    let t44427 = t3625 * t44425 * t12788;
    let t44431 = t12732 * t73;
    let t44484 = t3555 * t3766 * t5330;
    (t44418, t44420, t44421, t44425, t44427, t44431, t44484)
}
