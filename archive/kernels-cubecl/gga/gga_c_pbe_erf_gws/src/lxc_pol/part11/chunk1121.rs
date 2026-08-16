//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1121/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1121<F: Float>(t18240: F, t18243: F, t18245: F, t18261: F, t18267: F, t47862: F, t47864: F, t47866: F, t47868: F, t47870: F, t47872: F, t41208: F) -> (F, F) {
    let t47873 = t47862 - t47864 + t47866 - t47868 + t18240 - t18243 - t18245 + t18261 + t18267 + t47870 + t47872;
    let t47874 = F::cast_from(128.0_f64) / F::cast_from(45.0_f64) * t41208;
    (t47873, t47874)
}
