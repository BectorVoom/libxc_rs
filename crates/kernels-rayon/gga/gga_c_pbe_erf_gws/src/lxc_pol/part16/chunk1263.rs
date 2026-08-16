//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1263/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1263(t2409: f64, t26647: f64, t3959: f64, t8723: f64, t3202: f64, t3955: f64, t14121: f64, t26768: f64, t14113: f64, t14614: f64, t8797: f64, t8624: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53966 = t3959 * t2409 * t26647;
    let t53968 = t3959 * t8723;
    let t53970 = t3955 * t3202;
    let t53973 = t14121 * t2409 * t26768;
    let t53975 = t14113 * t14614;
    let t53981 = t3959 * t8797;
    let t53983 = t14121 * t8624;
    (t53966, t53968, t53970, t53973, t53975, t53981, t53983)
}
