//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1308/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1308(t52930: f64, t52961: f64, t52968: f64, t14311: f64, t3083: f64, t50927: f64, t50944: f64, t50949: f64, t52940: f64, t52944: f64, t52952: f64, t52956: f64, t52959: f64, t52976: f64, t52982: f64, t52986: f64) -> f64 {
    let t54896 = 7.0_f64 / 72.0_f64 * t52930;
    let t54902 = 7.0_f64 / 1152.0_f64 * t52961;
    let t54904 = 7.0_f64 / 576.0_f64 * t52968;
    let t54911 = 7.0_f64 / 144.0_f64 * t3083 * t14311;
    let t54912 = -t54896 + t52940 / 192.0_f64 + t52944 / 384.0_f64 - t52952 / 1536.0_f64 + t52956 / 384.0_f64 - t52959 / 96.0_f64 - t54902 + 7.0_f64 / 576.0_f64 * t50927 + t54904 + t52976 / 384.0_f64 - t52982 / 96.0_f64 - t52986 / 96.0_f64 + 7.0_f64 / 72.0_f64 * t50944 + 119.0_f64 / 1728.0_f64 * t50949 + t54911;
    t54912
}
