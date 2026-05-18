//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 726/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk726<F: Float>(t33293: F, t33294: F, t684: F, t33292: F, t240: F, t7513: F, t7242: F, t713: F, t7440: F, t7511: F, t7512: F, t1424: F, t6061: F) -> (F, F, F, F, F, F, F, F) {
    let t33296 = t33293 * t33294 * t684;
    let t33297 = t33292 * t33296;
    let t33300 = F::new(1.0) / t7513 / t240;
    let t33301 = t33300 * t7242;
    let t33302 = t7440 * t713;
    let t33303 = t33301 * t33302;
    let t33305 = t7511 * t7512 * t33303;
    let t33307 = t1424 * t6061;
    (t33296, t33297, t33300, t33301, t33302, t33303, t33305, t33307)
}
