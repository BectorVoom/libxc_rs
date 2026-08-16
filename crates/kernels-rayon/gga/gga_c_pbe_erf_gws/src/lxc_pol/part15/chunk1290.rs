//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1290/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1290(t3959: f64, t8723: f64, t3202: f64, t3955: f64, t14121: f64, t2409: f64, t26768: f64, t14113: f64, t14614: f64, t2242: f64, t4161: f64, t14742: f64, t840: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53968 = t3959 * t8723;
    let t53970 = t3955 * t3202;
    let t53971 = 7.0_f64 / 144.0_f64 * t53970;
    let t53973 = t14121 * t2409 * t26768;
    let t53975 = t14113 * t14614;
    let t53976 = 7.0_f64 / 576.0_f64 * t53975;
    let t53977 = t2242 * t4161;
    let t53980 = 7.0_f64 / 144.0_f64 * t840 * t14742;
    (t53968, t53971, t53973, t53976, t53977, t53980)
}
