//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1195/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1195(t21245: f64, t6244: f64, t6045: f64, t863: f64, t864: f64, t877: f64, t2308: f64, t6717: f64, t2206: f64, t6459: f64, t19859: f64, t20305: f64, t20727: f64, t20825: f64, t21227: f64, t21231: f64, t21239: f64, t21243: f64, t2266: f64, t2345: f64, t3247: f64, t6282: f64, t6366: f64, t6384: f64, t904: f64, t916: f64, t929: f64, t9425: f64) -> (f64, f64, f64, f64) {
    let t21246 = t21245 * t6244;
    let t21247 = 7.0_f64 / 12.0_f64 * t21246;
    let t21253 = t863 * t864 * t6045;
    let t21254 = t21253 * t877;
    let t21255 = 455.0_f64 / 324.0_f64 * t21254;
    let t21260 = t6717 * t2308;
    let t21266 = t2206 * t6459;
    let t21267 = 7.0_f64 / 4.0_f64 * t21266;
    let t21268 = t21231 + 15.0_f64 / 64.0_f64 * t3247 * t6366 * t6282 * t20825 - t21239 - t21243 + t21247 - 15.0_f64 / 64.0_f64 * t929 * t6384 * t904 * t19859 + t21255 + t9425 * t2345 * t20305 * t21227 / 8.0_f64 + 119.0_f64 / 576.0_f64 * t21260 + 7.0_f64 / 512.0_f64 * t2266 * t916 * t904 * t20727 - t21267;
    (t21247, t21255, t21267, t21268)
}
