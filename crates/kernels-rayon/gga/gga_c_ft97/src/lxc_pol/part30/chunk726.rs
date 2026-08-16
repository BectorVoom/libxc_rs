//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 726/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk726(t33293: f64, t33294: f64, t684: f64, t33292: f64, t240: f64, t7513: f64, t7242: f64, t713: f64, t7440: f64, t7511: f64, t7512: f64, t1424: f64, t6061: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33296 = t33293 * t33294 * t684;
    let t33297 = t33292 * t33296;
    let t33300 = 1.0_f64 / t7513 / t240;
    let t33301 = t33300 * t7242;
    let t33302 = t7440 * t713;
    let t33303 = t33301 * t33302;
    let t33305 = t7511 * t7512 * t33303;
    let t33307 = t1424 * t6061;
    (t33296, t33297, t33300, t33301, t33302, t33303, t33305, t33307)
}
