//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1099/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1099<F: Float>(t24088: F, t31643: F, t40255: F, t40262: F, t40264: F, t47420: F, t47423: F, t47426: F, t47430: F, t47476: F, t47479: F, t47482: F) -> F {
    let t47611 = F::new(0.6801e-1) * t47476 - F::new(0.11335e-1) * t47420 - F::new(0.15113333333333333333e-1) * t47479 - F::new(0.25188888888888888889e-2) * t40255 - F::new(0.2518888888888888889e-1) * t47423 + F::new(0.12594444444444444445e-1) * t47482 - F::new(0.78365432098765432099e-2) * t24088 + F::new(0.10075555555555555556e-1) * t40262 - F::new(0.15113333333333333333e-1) * t40264 - F::new(0.10075555555555555556e-1) * t31643 + F::new(0.55975308641975308645e-2) * t47426 + F::new(0.18891666666666666667e-2) * t47430;
    t47611
}
