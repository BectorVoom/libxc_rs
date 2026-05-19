//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 906/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk906<F: Float>(t10: F, t10089: F, t10090: F, t10094: F, t10096: F, t10097: F, t10130: F, t496: F, t5784: F, t5810: F, t8148: F, t8149: F, t8158: F, t8160: F) -> F {
    let t10132 = t10089 + t10090 + t8148 - F::new(0.195872e1) * t8149 + t8158 - F::new(0.97936e0) * t8160 - F::new(2.0) / F::new(9.0) * t5784 + t10094 - F::cast_from(0.97935999999999999999e0_f64) * t5810 - t10096 + F::new(3.0) / F::new(2.0) * t496 * t10 * t10097 + t10130;
    t10132
}
