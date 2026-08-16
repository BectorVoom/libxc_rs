//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 102/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk102(t154: f64, t277: f64, t52: f64, t273: f64, t133: f64, t158: f64, t230: f64, t265: f64, t267: f64) -> (f64, f64, f64, f64) {
    let t279 = t154 * t52 * t277;
    let t284 = 1.0_f64 / t273;
    let t285 = t133 * t284;
    let t287 = f64::exp(-(-t230 + t265 + t267) * t158 * t285);
    (t279, t284, t285, t287)
}
