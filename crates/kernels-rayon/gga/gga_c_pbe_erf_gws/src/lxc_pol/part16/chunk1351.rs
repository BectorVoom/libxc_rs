//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1351/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1351(t4227: f64, t6781: f64, t829: f64, t830: f64, t14886: f64, t4386: f64, t892: f64, t15036: f64, t19906: f64, t54463: f64, t14935: f64, t15021: f64, t15081: f64, t2376: f64, t2408: f64, t2409: f64, t29751: f64, t3066: f64, t51881: f64, t51896: f64, t52194: f64, t52551: f64, t54461: f64, t54465: f64, t54473: f64, t54484: f64, t6793: f64, t810: f64, t827: f64, t8734: f64, t8793: f64) -> f64 {
    let t55762 = t6781 * t4227;
    let t55764 = t829 * t830 * t55762;
    let t55769 = t4386 * t892 * t14886;
    let t55773 = 7.0_f64 / 72.0_f64 * t19906 * t15036;
    let t55781 = 7.0_f64 / 576.0_f64 * t54463;
    let t55795 = -t827 * t55764 / 48.0_f64 + 7.0_f64 / 72.0_f64 * t51881 + t6793 * t55769 / 24.0_f64 - t55773 + t8793 * t52194 / 24.0_f64 - 7.0_f64 / 1152.0_f64 * t51896 - t2408 * t29751 * t15021 / 12.0_f64 + t54461 / 1536.0_f64 - t55781 + t54465 / 24.0_f64 - t54473 / 192.0_f64 + 7.0_f64 / 144.0_f64 * t52551 + t54484 / 12.0_f64 + t2408 * t2409 * t2376 * t15081 * t810 / 24.0_f64 + t3066 * t2409 * t8734 * t14935 / 24.0_f64;
    t55795
}
