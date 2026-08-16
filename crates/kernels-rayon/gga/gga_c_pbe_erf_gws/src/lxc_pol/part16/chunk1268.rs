//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1268/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1268(t51351: f64, t9612: f64, t51350: f64, t6684: f64, t9641: f64, t3249: f64, t6238: f64, t899: f64, t923: f64, t2209: f64, t4026: f64, t863: f64) -> (f64, f64, f64, f64) {
    let t54045 = t51351 * t9612;
    let t54047 = t6684 * t51350;
    let t54048 = t54047 * t9641;
    let t54052 = t899 * t6238 * t923 * t3249;
    let t54055 = t863 * t4026 * t2209;
    (t54045, t54048, t54052, t54055)
}
