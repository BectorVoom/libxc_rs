//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 831/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk831(t6385: f64, t382: f64, t2053: f64, t944: f64, t2423: f64, t321: f64, t2429: f64, t2729: f64, t586: f64, t1660: f64, t197: f64, t1663: f64, param_gamma: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6865 = param_gamma * t6385;
    let t6866 = t6865 * t382;
    let t6868 = t944 * t2053;
    let t6870 = t321 * t6868 * t2423;
    let t6924 = t2429 * t944;
    let t7011 = t2729 * t586;
    let t7048 = t1660 * t197;
    let t7049 = t7048 * t1663;
    (t6865, t6866, t6868, t6870, t6924, t7011, t7049)
}
