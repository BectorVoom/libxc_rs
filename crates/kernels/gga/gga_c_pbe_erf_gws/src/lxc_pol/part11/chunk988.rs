//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 988/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk988<F: Float>(t10848: F, t3527: F, t1006: F, t12703: F, t12576: F, t2612: F, t12560: F, t7130: F, t18240: F, t18243: F, t18245: F, t18261: F, t18267: F, t47862: F, t47864: F, t41208: F) -> (F, F, F, F, F, F) {
    let t47866 = 8.0 / 15.0 * t10848 * t3527;
    let t47868 = 8.0 / 15.0 * t1006 * t12703;
    let t47870 = 32.0 / 15.0 * t2612 * t12576;
    let t47872 = 32.0 / 5.0 * t7130 * t12560;
    let t47873 = t47862 - t47864 + t47866 - t47868 + t18240 - t18243 - t18245 + t18261 + t18267 + t47870 + t47872;
    let t47874 = 128.0 / 45.0 * t41208;
    (t47866, t47868, t47870, t47872, t47873, t47874)
}
