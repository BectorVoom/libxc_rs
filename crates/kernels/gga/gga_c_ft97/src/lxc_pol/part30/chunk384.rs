//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 384/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk384<F: Float>(t2862: F, t319: F, t6278: F, t1476: F, t840: F, t882: F, t875: F, t871: F, t6260: F, t1497: F, t681: F, t89: F) -> (F, F, F, F, F, F) {
    let t6280 = t2862 * t319 * t6278;
    let t6284 = t840 * t882 * t1476;
    let t6287 = t1476 * t875;
    let t6289 = t840 * t871 * t6287;
    let t6293 = t840 * t319 * t6260;
    let t6298 = t89 * t681 * t1497 / F::cast_from(9.0_f64);
    (t6280, t6284, t6287, t6289, t6293, t6298)
}
