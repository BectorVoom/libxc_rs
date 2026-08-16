//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3180/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3180(t1227: f64, t248: f64, t45046: f64, t5971: f64, t15643: f64, t5005: f64, t15438: f64, t15453: f64, t15527: f64, t15555: f64, t15637: f64, t15737: f64, t19080: f64, t3496: f64, t44886: f64, t44890: f64, t44894: f64, t4582: f64, t5002: f64, t52776: f64, t52781: f64, t52792: f64, t52795: f64, t52801: f64, t62044: f64) -> f64 {
    let t65935 = t1227 * t248 * t45046 * t5971;
    let t65952 = t5005 * t15643;
    let t65954 = -t52776 / 72.0_f64 + t5002 * t15527 / 1536.0_f64 - t19080 * t3496 / 288.0_f64 - 5.0_f64 / 62208.0_f64 * t65935 - t52781 / 2304.0_f64 - t44886 / 13824.0_f64 - t44890 / 6912.0_f64 + t44894 / 13824.0_f64 + 5.0_f64 / 5184.0_f64 * t52792 - t52795 / 2304.0_f64 - 5.0_f64 / 5184.0_f64 * t1227 * t4582 * t15453 * t62044 + t15737 * t15555 / 384.0_f64 - t15438 * t15637 / 768.0_f64 + t52801 / 2304.0_f64 - t65952 / 864.0_f64;
    t65954
}
