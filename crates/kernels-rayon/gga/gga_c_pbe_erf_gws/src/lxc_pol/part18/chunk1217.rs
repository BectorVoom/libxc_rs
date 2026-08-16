//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1217/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1217(t6238: f64, t899: f64, t922: f64, t2250: f64, t3969: f64, t933: f64, t14022: f64, t828: f64, t2123: f64, t2209: f64, t4021: f64, t916: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51301 = t899 * t6238 * t922;
    let t51306 = t2250 * t3969 * t933;
    let t51328 = t14022 * t828;
    let t51329 = t51328 * t2123;
    let t51334 = t4021 * t2209;
    let t51350 = t3969 * t916;
    (t51301, t51306, t51328, t51329, t51334, t51350)
}
