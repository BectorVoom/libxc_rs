//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 957/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk957<F: Float>(t35697: F, t35699: F, t35703: F, t40354: F, t40357: F, t40360: F, t40362: F, t40365: F, t40367: F, t40372: F, t40377: F, t40379: F, t40384: F, t40389: F, t40391: F, t40396: F, t40401: F, t40403: F) -> F {
    let t40405 = F::new(0.59590439850616975157e-4) * t40354 + t40357 - F::new(0.1064114997332445985e-4) * t40360 - F::new(0.53205749866622299248e-5) * t40362 - F::new(0.42564599893297839398e-5) * t40365 - F::new(0.85129199786595678796e-5) * t40367 - F::new(0.31923449919973379548e-4) * t40372 - F::new(0.15961724959986689774e-4) * t40377 + F::new(0.31923449919973379548e-4) * t40379 + F::new(0.31923449919973379548e-4) * t40384 + F::new(0.15961724959986689774e-4) * t40389 + F::new(0.1064114997332445985e-4) * t40391 + F::new(0.1064114997332445985e-4) * t40396 + F::new(0.53205749866622299248e-5) * t40401 - F::new(0.1064114997332445985e-4) * t40403 - t35697 - t35699 - t35703;
    t40405
}
