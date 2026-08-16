//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1010/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1010(t11303: f64, t142: f64, t11290: f64, t11293: f64, t11296: f64, t11300: f64, t2037: f64, t5601: f64, t6036: f64, t6039: f64, t6043: f64, t6049: f64, t6050: f64, t6053: f64, t6058: f64, t6061: f64, t6064: f64, t8497: f64, t8502: f64, t8503: f64, t988: f64) -> f64 {
    let t11304 = t11303 * t142;
    let t11306 = -t6036 + t8502 + 0.79828226735030727293e-1_f64 * t8503 - 0.18218576931715098443e-4_f64 * t6039 - t6043 + t6049 - 0.53218817823353818195e-1_f64 * t6050 - t6053 - t6058 + 0.39914113367515363646e-1_f64 * t6061 + t6064 + 0.19957056683757681823e-1_f64 * t11290 - 3.0_f64 * t8497 * t11293 + 3.0_f64 * t11296 * t2037 + 6.0_f64 * t5601 * t11300 + t988 * t11304;
    t11306
}
