//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 721/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk721<F: Float>(t4859: F, t88: F, t36: F, t4259: F, t713: F, t762: F, t1597: F, t1917: F, t528: F, t220: F, t2735: F, t211: F, t1729: F, t586: F, t1791: F, t642: F) -> (F, F, F, F, F, F, F, F) {
    let t4860 = t4859 * t88;
    let t4862 = t36 * t4259;
    let t4863 = t4862 * t88;
    let t4864 = 120.0 * t4863;
    let t4872 = 0.66490888888888888888e-1 * t762 * t713;
    let t4873 = t1597 * t713;
    let t4876 = 0.9973633333333333333e-1 * t528 * t1917;
    let t4908 = t2735 * t220;
    let t4910 = 16.0 / 405.0 * t211 * t4908;
    let t4913 = t1729 * t586;
    let t4927 = t642 * t1791;
    (t4860, t4864, t4872, t4873, t4876, t4910, t4913, t4927)
}
