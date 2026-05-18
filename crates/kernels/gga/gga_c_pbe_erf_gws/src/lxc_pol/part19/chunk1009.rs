//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1009/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1009<F: Float>(t11303: F, t142: F, t11290: F, t11293: F, t11296: F, t11300: F, t2037: F, t5601: F, t6036: F, t6039: F, t6043: F, t6049: F, t6050: F, t6053: F, t6058: F, t6061: F, t6064: F, t8497: F, t8502: F, t8503: F, t988: F) -> F {
    let t11304 = t11303 * t142;
    let t11306 = -t6036 + t8502 + F::new(0.79828226735030727293e-1) * t8503 - F::new(0.18218576931715098443e-4) * t6039 - t6043 + t6049 - F::new(0.53218817823353818195e-1) * t6050 - t6053 - t6058 + F::new(0.39914113367515363646e-1) * t6061 + t6064 + F::new(0.19957056683757681823e-1) * t11290 - F::new(3.0) * t8497 * t11293 + F::new(3.0) * t11296 * t2037 + F::new(6.0) * t5601 * t11300 + t988 * t11304;
    t11306
}
