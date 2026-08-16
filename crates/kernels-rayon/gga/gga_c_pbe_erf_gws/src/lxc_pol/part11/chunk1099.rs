//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1099/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1099(t24088: f64, t31643: f64, t40255: f64, t40262: f64, t40264: f64, t47420: f64, t47423: f64, t47426: f64, t47430: f64, t47476: f64, t47479: f64, t47482: f64) -> f64 {
    let t47611 = 0.6801e-1_f64 * t47476 - 0.11335e-1_f64 * t47420 - 0.15113333333333333333e-1_f64 * t47479 - 0.25188888888888888889e-2_f64 * t40255 - 0.2518888888888888889e-1_f64 * t47423 + 0.12594444444444444445e-1_f64 * t47482 - 0.78365432098765432099e-2_f64 * t24088 + 0.10075555555555555556e-1_f64 * t40262 - 0.15113333333333333333e-1_f64 * t40264 - 0.10075555555555555556e-1_f64 * t31643 + 0.55975308641975308645e-2_f64 * t47426 + 0.18891666666666666667e-2_f64 * t47430;
    t47611
}
