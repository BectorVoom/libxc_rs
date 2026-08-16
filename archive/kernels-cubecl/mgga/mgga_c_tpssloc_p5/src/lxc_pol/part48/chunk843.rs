//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 843/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk843<F: Float>(t31337: F, t776: F, t23270: F, t22986: F, t1911: F, t7106: F, t2718: F, t1912: F, t24305: F, t30655: F, t30659: F, t30662: F, t30666: F, t30669: F, t31330: F, t31335: F, t6627: F, t7107: F, t855: F) -> (F, F, F, F) {
    let t31338 = t31337 * t776;
    let t31339 = t23270 * t31338;
    let t31340 = t22986 * t31339;
    let t31342 = t7106 * t1911;
    let t31343 = t2718 * t31342;
    let t31347 = -t24305 * t1912 - F::cast_from(0.82246703342411321825e-2_f64) * t31330 + F::cast_from(0.16449340668482264365e-1_f64) * t31335 - t30655 + F::cast_from(0.16449340668482264365e-1_f64) * t31340 + F::cast_from(2.0_f64) * t855 * t31343 - t30659 - t6627 * t7107 + t30662 - t30666 - t30669;
    (t31338, t31339, t31343, t31347)
}
