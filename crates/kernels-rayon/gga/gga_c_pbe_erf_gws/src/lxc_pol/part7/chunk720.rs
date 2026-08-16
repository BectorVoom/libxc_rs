//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 720/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk720(t542: f64, t671: f64, t670: f64, t1999: f64, t245: f64, t2003: f64, t5181: f64, t5183: f64, t5185: f64, t5209: f64, t5216: f64, t5223: f64, t5227: f64, t5277: f64, t5279: f64, t5282: f64, t5286: f64, t5290: f64, t5298: f64, t5303: f64, t5306: f64) -> (f64, f64, f64) {
    let t5917 = t542 * t671;
    let t5919 = 0.96187034332131941129e-1_f64 * t670 * t5917;
    let t5920 = t245 * t1999;
    let t5922 = 0.33545228223331014468e-1_f64 * t2003 * t5920;
    let t5923 = -t5181 + t5183 + t5185 + t5209 - t5919 + t5922 - t5216 - t5223 + t5227 - t5277 - t5279 - t5282 + t5286 + t5290 + t5298 + t5303 + t5306;
    (t5917, t5920, t5923)
}
