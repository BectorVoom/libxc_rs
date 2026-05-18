//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1195/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1195<F: Float>(t21245: F, t6244: F, t6045: F, t863: F, t864: F, t877: F, t2308: F, t6717: F, t2206: F, t6459: F, t19859: F, t20305: F, t20727: F, t20825: F, t21227: F, t21231: F, t21239: F, t21243: F, t2266: F, t2345: F, t3247: F, t6282: F, t6366: F, t6384: F, t904: F, t916: F, t929: F, t9425: F) -> (F, F, F, F) {
    let t21246 = t21245 * t6244;
    let t21247 = F::new(7.0) / F::new(12.0) * t21246;
    let t21253 = t863 * t864 * t6045;
    let t21254 = t21253 * t877;
    let t21255 = F::new(455.0) / F::new(324.0) * t21254;
    let t21260 = t6717 * t2308;
    let t21266 = t2206 * t6459;
    let t21267 = F::new(7.0) / F::new(4.0) * t21266;
    let t21268 = t21231 + F::new(15.0) / F::new(64.0) * t3247 * t6366 * t6282 * t20825 - t21239 - t21243 + t21247 - F::new(15.0) / F::new(64.0) * t929 * t6384 * t904 * t19859 + t21255 + t9425 * t2345 * t20305 * t21227 / F::new(8.0) + F::new(119.0) / F::new(576.0) * t21260 + F::new(7.0) / F::new(512.0) * t2266 * t916 * t904 * t20727 - t21267;
    (t21247, t21255, t21267, t21268)
}
