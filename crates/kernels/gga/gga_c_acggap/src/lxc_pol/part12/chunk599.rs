//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 599/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk599<F: Float>(t1131: F, t540: F, t960: F, t1313: F, t839: F, t922: F, t1137: F, t1324: F, t1140: F, t1328: F, t1322: F, t1350: F, t398: F, t429: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4479 = t540 * t1131;
    let t4480 = t960 * t4479;
    let t4483 = t1313 * t839;
    let t4484 = t960 * t4483;
    let t4487 = t1313 * t922;
    let t4488 = t960 * t4487;
    let t4492 = F::new(7.0) / F::new(72.0) * t1137 * t1324;
    let t4494 = F::new(7.0) / F::new(72.0) * t1140 * t1328;
    let t4495 = t1322 * t839;
    let t4496 = t960 * t4495;
    let t4503 = t398 * t429 * t1350;
    (t4479, t4480, t4483, t4484, t4487, t4488, t4492, t4494, t4495, t4496, t4503)
}
