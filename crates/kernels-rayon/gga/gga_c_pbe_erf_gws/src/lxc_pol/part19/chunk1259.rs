//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1259/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1259(t14959: f64, t4414: f64, t53545: f64, t20091: f64, t4209: f64, t53577: f64, t53583: f64, t53597: f64, t14911: f64, t2367: f64, t353: f64, t4228: f64, t4386: f64, t810: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t55228 = 7.0_f64 / 36.0_f64 * t4414 * t14959;
    let t55238 = 7.0_f64 / 288.0_f64 * t53545;
    let t55243 = t20091 * t4209;
    let t55248 = 7.0_f64 / 72.0_f64 * t53577;
    let t55251 = 7.0_f64 / 576.0_f64 * t53583;
    let t55258 = 7.0_f64 / 288.0_f64 * t53597;
    let t55279 = 7.0_f64 / 144.0_f64 * t2367 * t14911;
    let t55284 = t4386 * t353 * t4228 * t810;
    (t55228, t55238, t55243, t55248, t55251, t55258, t55279, t55284)
}
