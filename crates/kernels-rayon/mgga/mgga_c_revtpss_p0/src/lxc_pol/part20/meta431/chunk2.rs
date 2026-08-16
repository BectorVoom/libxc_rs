//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1626/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1626(t12901: f64, t13033: f64, t13042: f64, t13047: f64, t3172: f64, t3555: f64, t3781: f64, t5330: f64, t12861: f64, t12916: f64, t3718: f64, t11262: f64, t3600: f64, t3605: f64) -> (f64, f64, f64, f64, f64) {
    let t44658 = t13033 * t12901;
    let t44661 = t13042 * t3172 * t13047;
    let t44664 = t3555 * t3781 * t5330;
    let t44672 = t3718 * t12916 * t12861;
    let t44675 = t3600 * t11262 * t3605;
    (t44658, t44661, t44664, t44672, t44675)
}
