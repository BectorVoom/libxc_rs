//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 674/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk674<F: Float>(t272: F, t9606: F, t274: F, t668: F, t505: F, t123: F, t805: F, t194: F, t197: F, t8991: F, t815: F, t9636: F) -> (F, F, F, F, F, F) {
    let t10304 = F::cast_from(1.0_f64) / t272 / t9606;
    let t10327 = t274 * t668;
    let t10328 = t10327 * t505;
    let t10339 = t123 / t805 / t9606;
    let t10355 = t8991 / t197 / t194;
    let t10362 = t815 * t815;
    let t10363 = F::cast_from(1.0_f64) / t10362;
    let t10373 = F::cast_from(0.18521666970164609055e-1_f64) * t9636;
    (t10304, t10328, t10339, t10355, t10363, t10373)
}
