//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 966/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk966(t2932: f64, t751: f64, t2936: f64, t2033: f64, t2922: f64, t2986: f64, t5657: f64, t5888: f64, t6036: f64, t6039: f64, t6043: f64, t6049: f64, t6050: f64, t6053: f64, t6058: f64, t6059: f64, t6061: f64, t6064: f64, t988: f64) -> f64 {
    let t8502 = 0.39914113367515363646e-1_f64 * t751 * t2932;
    let t8503 = t751 * t2936;
    let t8514 = -t6036 + t8502 + 0.39914113367515363646e-1_f64 * t8503 - 0.36437153863430196886e-4_f64 * t6039 - t6043 + t6049 - 0.10643763564670763639e0_f64 * t6050 - t6053 - t6058 + 0.19957056683757681823e-1_f64 * t6059 + 0.79828226735030727292e-1_f64 * t6061 + t6064 + t988 * t5888 - 2.0_f64 * t2922 * t2033 + 3.0_f64 * t2986 * t5657;
    t8514
}
