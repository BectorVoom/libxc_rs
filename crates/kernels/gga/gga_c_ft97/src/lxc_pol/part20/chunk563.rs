//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 563/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk563<F: Float>(t10327: F, t505: F, t123: F, t805: F, t9606: F, t194: F, t197: F, t8991: F, t815: F, t287: F, t9636: F, t2740: F, t375: F, t89: F, t10: F, t296: F, t3050: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10328 = t10327 * t505;
    let t10339 = t123 / t805 / t9606;
    let t10355 = t8991 / t197 / t194;
    let t10362 = t815 * t815;
    let t10363 = 1.0 / t10362;
    let t10364 = t287 * t10363;
    let t10373 = 0.18521666970164609055e-1 * t9636;
    let t10394 = t89 * t375 * t2740;
    let t10397 = t10 * t3050 * t296;
    (t10328, t10339, t10355, t10362, t10363, t10364, t10373, t10394, t10397)
}
