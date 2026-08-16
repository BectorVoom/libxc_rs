//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 296/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk296<F: Float>(t1352: F, t384: F, t527: F, t935: F, t1: F, t483: F, t283: F, t1279: F, t1280: F, t659: F, t684: F, t693: F, t700: F, t711: F, t714: F, t753: F, t757: F, t805: F, t809: F) -> (F, F, F, F, F) {
    let t1353 = t384 * t1352;
    let t1355 = t935 * t527;
    let t1357 = t483 * t1;
    let t1358 = t1357 * t283;
    let t1359 = F::cast_from(0.18311447306006545054e-3_f64) * t1358;
    let t1360 = t659 - t684 - t693 + t700 - t1279 - t1280 + t711 - t714 - t1359 + t805 - t757 + t809 - t753;
    (t1353, t1355, t1357, t1358, t1360)
}
