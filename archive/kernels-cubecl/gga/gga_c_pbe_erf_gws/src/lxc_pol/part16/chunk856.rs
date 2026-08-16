//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 856/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk856<F: Float>(t1028: F, t1243: F, t4: F, t4573: F, t2706: F, t626: F, t656: F, t2698: F, t395: F, t2701: F, t5063: F, t954: F) -> (F, F, F, F, F, F, F, F) {
    let t7269 = t1243 * t1028;
    let t7271 = t4 * t4573;
    let t7272 = t7271 * t2706;
    let t7274 = t656 * t626;
    let t7278 = t395 * t2698;
    let t7279 = F::cast_from(0.15996296296296296296e-1_f64) * t7278;
    let t7280 = t395 * t2701;
    let t7282 = t5063 * t954;
    (t7269, t7271, t7272, t7274, t7278, t7279, t7280, t7282)
}
