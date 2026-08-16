//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 975/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk975<F: Float>(t143332: F, t1636: F, t7658: F, t89: F, t33988: F, t375: F, t33860: F, t6308: F, t681: F, t1486: F, t2399: F, t7650: F) -> (F, F, F, F, F, F) {
    let t143333 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t143332;
    let t143335 = t89 * t1636 * t7658;
    let t143336 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t143335;
    let t143339 = t89 * t375 * t33988;
    let t143355 = t6308 * t681 * t33860;
    let t143365 = t1486 * t2399 * t7650;
    (t143333, t143335, t143336, t143339, t143355, t143365)
}
