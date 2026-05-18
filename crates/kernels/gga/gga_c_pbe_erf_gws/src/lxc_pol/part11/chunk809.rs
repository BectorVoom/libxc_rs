//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 809/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk809<F: Float>(t12754: F, t12756: F, t12758: F, t12759: F, t12760: F, t12761: F, t12763: F, t12764: F, t12765: F, t12769: F, t12771: F, t5384: F, t5387: F, t5423: F, t5429: F, t5433: F, t7734: F, t7736: F) -> F {
    let t13023 = t12754 + t12756 + t12758 + t12759 - t5384 + t5387 + t7734 + F::new(0.36466666666666666665e0) * t7736 - t12760 - t12761 + t12763 + t12764 + t12765 + t12769 - t12771 + t5423 + t5429 + t5433;
    t13023
}
