//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1278/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1278(t14567: f64, t2080: f64, t9365: f64, t2134: f64, t8897: f64, t51267: f64, t8983: f64, t14007: f64, t9334: f64, t51470: f64, t9338: f64, t14498: f64, t9671: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54186 = t2080 * t9365 * t14567;
    let t54188 = t2134 * t8897;
    let t54190 = t51267 * t8983;
    let t54192 = t14007 * t9334;
    let t54194 = t51470 * t9338;
    let t54196 = t14498 * t9671;
    (t54186, t54188, t54190, t54192, t54194, t54196)
}
