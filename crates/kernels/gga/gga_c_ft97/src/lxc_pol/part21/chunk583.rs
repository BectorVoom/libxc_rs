//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 583/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk583<F: Float>(t11176: F, t3053: F, t371: F, t7876: F, t1630: F, t929: F, t1593: F, t2993: F, t7705: F, t419: F, t173: F, t1736: F, t2984: F, t420: F, t8119: F, t2248: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11177 = t11176 * t3053;
    let t11232 = t371 * t7876;
    let t11233 = t1630 * t929;
    let t11247 = t1593 * t929;
    let t11259 = t7705 * t2993;
    let t11260 = t419 * t11259;
    let t11262 = t173 * t1736;
    let t11263 = t11262 * t2984;
    let t11264 = t419 * t11263;
    let t11265 = 0.56749874115226337448e-2 * t11264;
    let t11269 = t420 * t8119;
    let t11273 = t2248 * t1736;
    (t11177, t11232, t11233, t11247, t11260, t11262, t11264, t11265, t11269, t11273)
}
