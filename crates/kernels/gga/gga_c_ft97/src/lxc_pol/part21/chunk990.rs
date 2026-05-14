//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 990/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk990<F: Float>(t39: F, t8051: F, t22563: F, t6: F, t428: F, t71: F, t420: F, t53: F, t401: F, t8907: F, t142: F, t7367: F, t1008: F, t2: F, t7242: F, t369: F, t7954: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32167 = t8051 * t39;
    let t32250 = t22563 * t6;
    let t32260 = t71 * t428;
    let t32261 = t420 * t32260;
    let t32268 = t71 * t53;
    let t32269 = t420 * t32268;
    let t32273 = t71 * t401;
    let t32274 = t420 * t32273;
    let t32772 = t8907 * t39;
    let t32905 = 1.0 / t7367 / t142;
    let t34871 = t71 * t1008;
    let t36452 = t7242 * t2;
    let t37305 = t7954 * t369;
    (t32167, t32250, t32261, t32269, t32274, t32772, t32905, t34871, t36452, t37305)
}
