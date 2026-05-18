//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 296/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk296<F: Float>(t1168: F, t1174: F, t1453: F, t1454: F, t1455: F, t1459: F, t1462: F, t1525: F, t458: F, t462: F, t598: F) -> F {
    let t1528 = t1453 + t1454 * t1455 + t598 * t1168 / F::new(4.0) + t458 * t1459 / F::new(4.0) - F::new(5.0) / F::new(16.0) * t1174 * t1462 + t462 * t1525 / F::new(4.0);
    t1528
}
