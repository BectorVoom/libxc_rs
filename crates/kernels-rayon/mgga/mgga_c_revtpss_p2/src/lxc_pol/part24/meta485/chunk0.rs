//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1477/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1477(t3670: f64, t6594: f64, t3718: f64, t44546: f64, t6689: f64, t3717: f64, t70994: f64, t3617: f64, t6587: f64, t3147: f64, t6593: f64, t3594: f64, t3597: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t71280 = t3670 * t6594;
    let t71294 = t3718 * t44546 * t6689;
    let t71513 = t3717 * t70994;
    let t71543 = t3617 * t6587;
    let t71691 = t6593 * t3147;
    let t71693 = t3594 * t3597 * t71691;
    (t71280, t71294, t71513, t71543, t71691, t71693)
}
