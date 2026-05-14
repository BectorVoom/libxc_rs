//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 600/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk600<F: Float>(t28149: F, t28193: F, t28233: F, t28281: F, t28325: F, t28372: F, t28413: F, t28458: F, t24191: F, t6752: F, t193: F, t375: F, t7087: F, t89: F, t6308: F, t681: F, t7063: F) -> (F, F, F, F) {
    let t28461 = t28149 + t28193 + t28233 + t28281 + t28325 + t28372 + t28413 + t28458;
    let t28466 = t24191 * t6752;
    let t28467 = t193 * t28466;
    let t28491 = t89 * t375 * t7087;
    let t28494 = t6308 * t681 * t7063;
    (t28461, t28467, t28491, t28494)
}
