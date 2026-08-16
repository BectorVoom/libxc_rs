//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 809/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk809(t12754: f64, t12756: f64, t12758: f64, t12759: f64, t12760: f64, t12761: f64, t12763: f64, t12764: f64, t12765: f64, t12769: f64, t12771: f64, t5384: f64, t5387: f64, t5423: f64, t5429: f64, t5433: f64, t7734: f64, t7736: f64) -> f64 {
    let t13023 = t12754 + t12756 + t12758 + t12759 - t5384 + t5387 + t7734 + 0.36466666666666666665e0_f64 * t7736 - t12760 - t12761 + t12763 + t12764 + t12765 + t12769 - t12771 + t5423 + t5429 + t5433;
    t13023
}
