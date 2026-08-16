//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 720/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk720<F: Float>(t542: F, t671: F, t670: F, t1999: F, t245: F, t2003: F, t5181: F, t5183: F, t5185: F, t5209: F, t5216: F, t5223: F, t5227: F, t5277: F, t5279: F, t5282: F, t5286: F, t5290: F, t5298: F, t5303: F, t5306: F) -> (F, F, F) {
    let t5917 = t542 * t671;
    let t5919 = F::cast_from(0.96187034332131941129e-1_f64) * t670 * t5917;
    let t5920 = t245 * t1999;
    let t5922 = F::cast_from(0.33545228223331014468e-1_f64) * t2003 * t5920;
    let t5923 = -t5181 + t5183 + t5185 + t5209 - t5919 + t5922 - t5216 - t5223 + t5227 - t5277 - t5279 - t5282 + t5286 + t5290 + t5298 + t5303 + t5306;
    (t5917, t5920, t5923)
}
