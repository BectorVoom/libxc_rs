//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 808/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk808<F: Float>(t10841: F, t12715: F, t12719: F, t12721: F, t12725: F, t12726: F, t12728: F, t12733: F, t12735: F, t12737: F, t12739: F, t12741: F, t12744: F, t12746: F, t12750: F, t5359: F, t7573: F) -> F {
    let t13021 = t12715 - t12719 + t12721 + t12725 + F::cast_from(0.9973633333333333333e-1_f64) * t7573 - t12726 - t12728 + t12733 - t12735 + t12737 + t12739 - t12741 + t5359 - t12744 + t12746 + F::new(2.0) / F::new(3.0) * t10841 - t12750;
    t13021
}
