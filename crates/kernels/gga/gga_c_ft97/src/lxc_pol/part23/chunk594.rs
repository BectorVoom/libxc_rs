//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 594/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk594<F: Float>(t191: F, t7640: F, t793: F, t89: F, t9733: F, t272: F, t9606: F, t274: F, t668: F, t505: F, t123: F, t805: F, t194: F, t197: F, t8991: F, t815: F) -> (F, F, F, F, F, F, F) {
    let t10261 = t191 * t7640;
    let t10279 = t89 * t9733 * t793;
    let t10304 = 1.0 / t272 / t9606;
    let t10327 = t274 * t668;
    let t10328 = t10327 * t505;
    let t10339 = t123 / t805 / t9606;
    let t10355 = t8991 / t197 / t194;
    let t10362 = t815 * t815;
    (t10261, t10279, t10304, t10328, t10339, t10355, t10362)
}
